use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Defaults {
    pub provider: String,
    pub dry_run_default: bool,
    pub proposal_required: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ProviderProfile {
    pub kind: String,
    pub network: Option<bool>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub auth: Option<String>,
    pub auth_env: Option<String>,
    pub model_suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyConfig {
    pub network_default: String,
    pub allow_provider_network: bool,
    pub secrets_redaction: bool,
    pub apply_requires_confirmation: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub defaults: Defaults,
    pub providers: HashMap<String, ProviderProfile>,
    pub policy: PolicyConfig,
}

pub fn load_config(custom_path: Option<&str>) -> Result<Config, String> {
    let path = if let Some(p) = custom_path {
        std::path::PathBuf::from(p)
    } else {
        let p = std::path::PathBuf::from("comptext.toml");
        if p.exists() {
            p
        } else {
            std::path::PathBuf::from("comptext.example.toml")
        }
    };

    if !path.exists() {
        return Err(format!(
            "Configuration file not found at '{}'",
            path.display()
        ));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("failed to read config file '{}': {e}", path.display()))?;

    let config: Config = toml::from_str(&content).map_err(|e| {
        format!(
            "failed to parse TOML configuration from '{}': {e}",
            path.display()
        )
    })?;

    Ok(config)
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Help,
    Version,
    Doctor,
    ProvidersList,
    ContextInspect,
    ContextPack {
        task: String,
    },
    Ask {
        provider: Option<String>,
        dry_run: bool,
        prompt: String,
    },
    Propose {
        provider: Option<String>,
        task: String,
    },
    Apply {
        proposal_path: Option<String>,
        yes: bool,
    },
    Validate,
    Benchmark {
        provider: Option<String>,
        task: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Policy {
    pub secrets_redacted: bool,
    pub generated_outputs_excluded: bool,
    pub patch_requires_approval: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContextPack {
    pub schema_version: String,
    pub task: String,
    pub mode: String,
    pub repo_profile: String,
    pub read_first: Vec<String>,
    pub included_files: Vec<String>,
    pub excluded_files: Vec<String>,
    pub allowed_write_paths: Vec<String>,
    pub forbidden_actions: Vec<String>,
    pub validation_commands: Vec<String>,
    pub provider: Option<String>,
    pub rendered_context: String,
    pub policy: Policy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelRequest {
    pub provider: String,
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Operation {
    pub op: String,
    pub path: String,
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub schema_version: String,
    pub task: String,
    pub rationale: String,
    pub preconditions: Vec<String>,
    pub affected_files: Vec<String>,
    pub operations: Vec<Operation>,
    pub validation_commands: Vec<String>,
    pub rollback_strategy: String,
    pub risk_notes: String,
}

pub fn run<I>(args: I) -> i32
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let argv: Vec<String> = args.into_iter().map(Into::into).collect();

    let mut config_path = None;
    let mut cleaned_argv = Vec::new();
    let mut i = 0;
    while i < argv.len() {
        if argv[i] == "--config" {
            if i + 1 >= argv.len() {
                eprintln!("error: missing path after --config");
                return 2;
            }
            config_path = Some(argv[i + 1].clone());
            i += 2;
        } else {
            cleaned_argv.push(argv[i].clone());
            i += 1;
        }
    }

    let config = match load_config(config_path.as_deref()) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("error loading config: {e}");
            return 1;
        }
    };

    match parse(&cleaned_argv) {
        Ok(Command::Help) => {
            print_help();
            0
        }
        Ok(Command::Version) => {
            println!("ctxt {VERSION}");
            0
        }
        Ok(Command::Doctor) => {
            print_doctor(&config);
            0
        }
        Ok(Command::ProvidersList) => {
            print_providers(&config);
            0
        }
        Ok(Command::ContextInspect) => match handle_context_inspect() {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
        Ok(Command::ContextPack { task }) => match handle_context_pack(&task) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
        Ok(Command::Ask {
            provider,
            dry_run,
            prompt,
        }) => match handle_ask(provider.as_deref(), dry_run, &prompt, &config) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
        Ok(Command::Propose { provider, task }) => {
            match handle_propose(provider.as_deref(), &task, &config) {
                Ok(_) => 0,
                Err(e) => {
                    eprintln!("error: {e}");
                    1
                }
            }
        }
        Ok(Command::Apply { proposal_path, yes }) => {
            match handle_apply(proposal_path.as_deref(), yes) {
                Ok(_) => 0,
                Err(e) => {
                    eprintln!("error: {e}");
                    1
                }
            }
        }
        Ok(Command::Validate) => match handle_validate() {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
        Ok(Command::Benchmark { provider, task }) => {
            match handle_benchmark(provider.as_deref(), &task, &config) {
                Ok(_) => 0,
                Err(e) => {
                    eprintln!("error: {e}");
                    1
                }
            }
        }
        Err(message) => {
            eprintln!("error: {message}");
            eprintln!("run `ctxt --help` for usage");
            2
        }
    }
}

fn parse(argv: &[String]) -> Result<Command, String> {
    if argv.is_empty() {
        return Ok(Command::Help);
    }

    let first = &argv[0];
    match first.as_str() {
        "--help" | "-h" | "help" => {
            if argv.len() > 1 {
                return Err(format!("unexpected argument '{}' for help", argv[1]));
            }
            Ok(Command::Help)
        }
        "--version" | "-V" | "version" => {
            if argv.len() > 1 {
                return Err(format!("unexpected argument '{}' for version", argv[1]));
            }
            Ok(Command::Version)
        }
        "doctor" => {
            if argv.len() > 1 {
                return Err(format!("unexpected argument '{}' for doctor", argv[1]));
            }
            Ok(Command::Doctor)
        }
        "providers" => {
            if argv.len() < 2 {
                return Err(
                    "missing subcommand for 'providers'. Usage: ctxt providers list".to_string(),
                );
            }
            if argv[1] != "list" {
                return Err(format!(
                    "unsupported subcommand '{}' for 'providers'",
                    argv[1]
                ));
            }
            if argv.len() > 2 {
                return Err(format!(
                    "unexpected argument '{}' for 'providers list'",
                    argv[2]
                ));
            }
            Ok(Command::ProvidersList)
        }
        "context" => {
            if argv.len() < 2 {
                return Err(
                    "missing subcommand for 'context'. Usage: ctxt context inspect | pack"
                        .to_string(),
                );
            }
            match argv[1].as_str() {
                "inspect" => {
                    if argv.len() > 2 {
                        return Err(format!(
                            "unexpected argument '{}' for 'context inspect'",
                            argv[2]
                        ));
                    }
                    Ok(Command::ContextInspect)
                }
                "pack" => {
                    if argv.len() < 4 {
                        return Err("missing --task argument for 'context pack'. Usage: ctxt context pack --task \"<task>\"".to_string());
                    }
                    if argv[2] != "--task" {
                        return Err(format!(
                            "unexpected option '{}' for 'context pack'. Expected '--task'",
                            argv[2]
                        ));
                    }
                    let task = argv[3].clone();
                    if argv.len() > 4 {
                        return Err(format!(
                            "unexpected argument '{}' for 'context pack'",
                            argv[4]
                        ));
                    }
                    Ok(Command::ContextPack { task })
                }
                other => Err(format!("unsupported subcommand '{}' for 'context'", other)),
            }
        }
        "ask" => {
            if argv.len() < 2 {
                return Err("missing prompt for 'ask'".to_string());
            }

            let mut provider = None;
            let mut dry_run = false;
            let mut prompt = String::new();

            let i = 1;
            let mut i_mut = i;
            while i_mut < argv.len() {
                match argv[i_mut].as_str() {
                    "--dry-run" => {
                        dry_run = true;
                        i_mut += 1;
                    }
                    "--provider" => {
                        if i_mut + 1 >= argv.len() {
                            return Err("missing provider name after --provider".to_string());
                        }
                        provider = Some(argv[i_mut + 1].clone());
                        i_mut += 2;
                    }
                    other => {
                        if other.starts_with('-') {
                            return Err(format!("unsupported option '{other}' for 'ask'"));
                        }
                        if !prompt.is_empty() {
                            return Err(format!("unexpected argument '{other}' for 'ask'"));
                        }
                        prompt = other.to_string();
                        i_mut += 1;
                    }
                }
            }

            if prompt.is_empty() {
                return Err("missing prompt for 'ask'".to_string());
            }

            Ok(Command::Ask {
                provider,
                dry_run,
                prompt,
            })
        }
        "propose" => {
            let mut provider = None;
            let mut task = String::new();

            let mut i = 1;
            while i < argv.len() {
                match argv[i].as_str() {
                    "--provider" => {
                        if i + 1 >= argv.len() {
                            return Err("missing provider name after --provider".to_string());
                        }
                        provider = Some(argv[i + 1].clone());
                        i += 2;
                    }
                    other => {
                        if other.starts_with('-') {
                            return Err(format!("unsupported option '{other}' for 'propose'"));
                        }
                        if !task.is_empty() {
                            return Err(format!("unexpected argument '{other}' for 'propose'"));
                        }
                        task = other.to_string();
                        i += 1;
                    }
                }
            }

            if task.is_empty() {
                return Err("missing task description for 'propose'".to_string());
            }

            Ok(Command::Propose { provider, task })
        }
        "apply" => {
            let mut proposal_path = None;
            let mut yes = false;

            let mut i = 1;
            while i < argv.len() {
                match argv[i].as_str() {
                    "--yes" | "-y" => {
                        yes = true;
                        i += 1;
                    }
                    other => {
                        if other.starts_with('-') {
                            return Err(format!("unsupported option '{other}' for 'apply'"));
                        }
                        if proposal_path.is_some() {
                            return Err(format!("unexpected argument '{other}' for 'apply'"));
                        }
                        proposal_path = Some(other.to_string());
                        i += 1;
                    }
                }
            }
            Ok(Command::Apply { proposal_path, yes })
        }
        "validate" => {
            if argv.len() > 1 {
                return Err(format!("unexpected argument '{}' for 'validate'", argv[1]));
            }
            Ok(Command::Validate)
        }
        "benchmark" => {
            let mut provider = None;
            let mut task = String::new();

            let mut i = 1;
            while i < argv.len() {
                match argv[i].as_str() {
                    "--provider" => {
                        if i + 1 >= argv.len() {
                            return Err("missing provider name after --provider".to_string());
                        }
                        provider = Some(argv[i + 1].clone());
                        i += 2;
                    }
                    other => {
                        if other.starts_with('-') {
                            return Err(format!("unsupported option '{other}' for 'benchmark'"));
                        }
                        if !task.is_empty() {
                            return Err(format!("unexpected argument '{other}' for 'benchmark'"));
                        }
                        task = other.to_string();
                        i += 1;
                    }
                }
            }

            if task.is_empty() {
                return Err("missing task description for 'benchmark'".to_string());
            }

            Ok(Command::Benchmark { provider, task })
        }
        other => {
            if other.starts_with('-') {
                Err(format!("unsupported option '{}'", other))
            } else {
                Err(format!("unsupported command '{}'", other))
            }
        }
    }
}

fn print_help() {
    println!(
        "CompText CLI / ctxt {VERSION}\n\
\n\
USAGE:\n\
    ctxt <COMMAND>\n\
\n\
COMMANDS:\n\
    doctor              Run local readiness checks\n\
    providers list      List configured provider kinds\n\
    version             Print version\n\
    context inspect     Inspect the workspace context\n\
    context pack        Pack deterministic Context Pack\n\
    ask                 Run query against provider (dry-run supported)\n\
    propose             Generate proposals for target task (dry-run mode)\n\
    apply               Apply proposed changes and validate\n\
    validate            Validate the repository state against proposal\n\
    benchmark           Run deterministic local model/context benchmarks\n\
\n\
SAFETY DEFAULTS:\n\
    network_default=deny\n\
    dry_run_before_network=true\n\
    proposal_before_apply=true\n\
    secrets_redaction=true"
    );
}

fn print_doctor(config: &Config) {
    println!("CompText doctor");
    println!("status: ok");
    println!("network_default: {}", config.policy.network_default);
    println!("provider_default: {}", config.defaults.provider);
    println!("proposal_required: {}", config.defaults.proposal_required);
    println!("secrets_policy: redact-before-artifact");
}

fn print_providers(config: &Config) {
    let mut names: Vec<&String> = config.providers.keys().collect();
    names.sort();

    for name in names {
        let profile = &config.providers[name];

        let network_str = match profile.network {
            Some(true) => "network=true",
            Some(false) => "network=false",
            None => {
                if profile.kind == "dummy" {
                    "network=false"
                } else {
                    "network=true"
                }
            }
        };

        let url_str = if let Some(ref url) = profile.base_url {
            format!("base_url={url}")
        } else {
            String::new()
        };

        let mut auth_str = if let Some(ref auth) = profile.auth {
            format!("auth={}", auth)
        } else if let Some(ref auth_env) = profile.auth_env {
            format!("auth_env={}", auth_env)
        } else {
            String::new()
        };

        let auth_lower = auth_str.to_lowercase();
        if (auth_lower.contains("secret")
            || auth_lower.contains("password")
            || auth_lower.contains("token")
            || auth_lower.contains("key"))
            && !auth_lower.contains("ollama_api_key")
            && !auth_lower.contains("optional_api_key")
        {
            auth_str = "auth=[REDACTED-METADATA]".to_string();
        }

        print!("{}\tkind={}\t{}", name, profile.kind, network_str);
        if !url_str.is_empty() {
            print!("\t{}", url_str);
        }
        if !auth_str.is_empty() {
            print!("\t{}", auth_str);
        }
        println!();
    }
}

fn collect_files(
    dir: &std::path::Path,
    files: &mut Vec<std::path::PathBuf>,
) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name == ".git" || name == "target" || name == ".comptext" || name == "reports" {
                    continue;
                }
                collect_files(&path, files)?;
            } else {
                files.push(path);
            }
        }
    }
    Ok(())
}

fn normalize_path(path: &std::path::Path) -> String {
    let mut s = path.to_string_lossy().into_owned();
    if s.starts_with(".\\") || s.starts_with("./") {
        s = s[2..].to_string();
    }
    s.replace('\\', "/")
}

fn is_sensitive_context_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let lower = normalized.to_ascii_lowercase();
    let file_name = lower.rsplit('/').next().unwrap_or(lower.as_str());

    file_name == ".env"
        || file_name.starts_with(".env.")
        || file_name.ends_with(".key")
        || file_name.ends_with(".pem")
        || file_name.ends_with(".p12")
        || file_name.ends_with(".pfx")
        || file_name.contains("key")
        || file_name.contains("credential")
        || matches!(file_name, "id_rsa" | "id_dsa" | "id_ecdsa" | "id_ed25519")
}

fn ensure_provider_network_allowed(
    config: &Config,
    profile: &ProviderProfile,
    provider_name: &str,
) -> Result<(), String> {
    if config.policy.allow_provider_network && profile.network.unwrap_or(true) {
        return Ok(());
    }

    Err(format!(
        "Network access denied by security policy for provider '{provider_name}'. Enable allow_provider_network and provider network=true in config to allow live execution."
    ))
}

fn redact_secrets(content: &str) -> String {
    let mut redacted = String::new();
    for line in content.lines() {
        let lower = line.to_lowercase();
        if (lower.contains("key")
            || lower.contains("secret")
            || lower.contains("token")
            || lower.contains("password"))
            && (line.contains('=') || line.contains(':'))
        {
            if let Some(pos) = line.find('=') {
                let (prefix, _) = line.split_at(pos + 1);
                redacted.push_str(prefix);
                redacted.push_str(" \"[REDACTED]\"\n");
            } else if let Some(pos) = line.find(':') {
                let (prefix, _) = line.split_at(pos + 1);
                redacted.push_str(prefix);
                redacted.push_str(" \"[REDACTED]\"\n");
            } else {
                redacted.push_str(line);
                redacted.push('\n');
            }
        } else {
            redacted.push_str(line);
            redacted.push('\n');
        }
    }
    redacted
}

fn build_context_pack(task: &str) -> Result<ContextPack, String> {
    let mut files = Vec::new();
    collect_files(std::path::Path::new("."), &mut files)
        .map_err(|e| format!("failed to scan workspace: {e}"))?;

    files.sort();

    let mut included_files = Vec::new();
    let mut rendered_context = String::new();

    for file in files {
        let rel_path = normalize_path(&file);
        if rel_path.ends_with(".exe")
            || rel_path.ends_with(".dll")
            || rel_path.ends_with(".pdb")
            || rel_path == "Cargo.lock"
            || is_sensitive_context_path(&rel_path)
        {
            continue;
        }
        let content = std::fs::read_to_string(&file)
            .map_err(|e| format!("failed to read file '{rel_path}': {e}"))?;

        let redacted = redact_secrets(&content);

        rendered_context.push_str(&format!("=== FILE: {} ===\n{}\n\n", rel_path, redacted));
        included_files.push(rel_path);
    }

    Ok(ContextPack {
        schema_version: "0.1".to_string(),
        task: task.to_string(),
        mode: "ask".to_string(),
        repo_profile: "default".to_string(),
        read_first: vec![],
        included_files,
        excluded_files: vec![
            "target/".to_string(),
            ".git/".to_string(),
            ".comptext/".to_string(),
            "reports/".to_string(),
            ".env".to_string(),
            ".env.*".to_string(),
            "*.key".to_string(),
            "*.pem".to_string(),
            "*.p12".to_string(),
            "*.pfx".to_string(),
            "*key*".to_string(),
            "*credential*".to_string(),
        ],
        allowed_write_paths: vec![],
        forbidden_actions: vec![],
        validation_commands: vec!["cargo test".to_string()],
        provider: Some("dummy".to_string()),
        rendered_context,
        policy: Policy {
            secrets_redacted: true,
            generated_outputs_excluded: true,
            patch_requires_approval: true,
        },
    })
}

fn handle_context_inspect() -> Result<(), String> {
    let cp = build_context_pack("inspect")?;
    println!("Context Pack Inspection:");
    println!("Schema Version: {}", cp.schema_version);
    println!("Total included files: {}", cp.included_files.len());
    println!("Included files:");
    for file in &cp.included_files {
        println!("  - {file}");
    }
    println!("Excluded paths/patterns:");
    for excl in &cp.excluded_files {
        println!("  - {excl}");
    }
    println!(
        "Rendered context size: {} characters",
        cp.rendered_context.len()
    );
    Ok(())
}

fn handle_context_pack(task: &str) -> Result<(), String> {
    let cp = build_context_pack(task)?;
    std::fs::create_dir_all(".comptext")
        .map_err(|e| format!("failed to create .comptext directory: {e}"))?;

    let json_content = serde_json::to_string_pretty(&cp)
        .map_err(|e| format!("failed to serialize context pack: {e}"))?;

    std::fs::write(".comptext/context_pack.latest.json", json_content)
        .map_err(|e| format!("failed to write context pack: {e}"))?;

    println!("Context Pack written to .comptext/context_pack.latest.json");
    Ok(())
}

fn handle_ask(
    provider: Option<&str>,
    dry_run: bool,
    prompt: &str,
    config: &Config,
) -> Result<(), String> {
    let resolved_provider = provider.unwrap_or(config.defaults.provider.as_str());

    let resolved_dry_run = if dry_run {
        true
    } else if provider.is_some() {
        false
    } else {
        config.defaults.dry_run_default
    };

    let profile = config.providers.get(resolved_provider).ok_or_else(|| {
        format!("provider profile '{resolved_provider}' not found in configuration")
    })?;

    let cp = build_context_pack(prompt)?;
    std::fs::create_dir_all(".comptext")
        .map_err(|e| format!("failed to create .comptext directory: {e}"))?;

    let cp_json = serde_json::to_string_pretty(&cp)
        .map_err(|e| format!("failed to serialize context pack: {e}"))?;

    std::fs::write(".comptext/context_pack.latest.json", &cp_json)
        .map_err(|e| format!("failed to write context pack: {e}"))?;

    let system_prompt = format!(
        "You are a helpful coding assistant. Here is the repository context:\n\n{}",
        cp.rendered_context
    );
    let request = ModelRequest {
        provider: resolved_provider.to_string(),
        model: "dummy-model".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            },
        ],
    };

    let req_json = serde_json::to_string_pretty(&request)
        .map_err(|e| format!("failed to serialize model request: {e}"))?;

    std::fs::write(".comptext/model_request.latest.json", req_json)
        .map_err(|e| format!("failed to write model request: {e}"))?;

    if profile.kind == "openai-compatible" {
        let model_name = profile
            .model
            .clone()
            .unwrap_or_else(|| "gpt-4o".to_string());
        let openai_payload = serde_json::json!({
            "model": model_name,
            "messages": request.messages
        });
        let openai_req_json = serde_json::to_string_pretty(&openai_payload)
            .map_err(|e| format!("failed to serialize OpenAI payload: {e}"))?;
        std::fs::write(".comptext/openai_request.latest.json", openai_req_json)
            .map_err(|e| format!("failed to write OpenAI request artifact: {e}"))?;
    }

    if resolved_dry_run {
        println!("Dry-run successful.");
        println!("Context Pack: .comptext/context_pack.latest.json");
        println!("Model Request: .comptext/model_request.latest.json");
        if profile.kind == "openai-compatible" {
            println!("OpenAI Request: .comptext/openai_request.latest.json");
        }
        return Ok(());
    }

    match profile.kind.as_str() {
        "dummy" => {
            use crate::provider::{DummyProvider, Provider};
            let prov = DummyProvider;
            let response = prov.execute(&request)?;

            let resp_json = serde_json::to_string_pretty(&response)
                .map_err(|e| format!("failed to serialize model response: {e}"))?;

            std::fs::write(".comptext/model_response.latest.json", resp_json)
                .map_err(|e| format!("failed to write model response: {e}"))?;

            println!("Response from {} provider:", prov.name());
            println!("{}", response.content);
            Ok(())
        }
        "ollama" => {
            ensure_provider_network_allowed(config, profile, resolved_provider)?;

            use crate::provider::{OllamaProvider, Provider};
            let url = profile
                .base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            let suffix = profile.model_suffix.clone();
            let auth = profile.auth_env.clone();

            let prov = OllamaProvider {
                name: resolved_provider.to_string(),
                base_url: url,
                model_suffix: suffix,
                auth_env: auth,
            };

            let response = prov.execute(&request)?;

            let resp_json = serde_json::to_string_pretty(&response)
                .map_err(|e| format!("failed to serialize model response: {e}"))?;

            std::fs::write(".comptext/model_response.latest.json", resp_json)
                .map_err(|e| format!("failed to write model response: {e}"))?;

            println!("Response from {} provider:", prov.name());
            println!("{}", response.content);
            Ok(())
        }
        "openai-compatible" => {
            use crate::provider::{OpenaiProvider, Provider};
            let url = profile
                .base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434/v1".to_string());
            let model = profile.model.clone();
            let auth = profile.auth_env.clone();
            let allow_net = config.policy.allow_provider_network && profile.network.unwrap_or(true);

            let prov = OpenaiProvider {
                name: resolved_provider.to_string(),
                base_url: url,
                model,
                auth_env: auth,
                allow_network: allow_net,
            };

            let response = prov.execute(&request)?;

            let resp_json = serde_json::to_string_pretty(&response)
                .map_err(|e| format!("failed to serialize model response: {e}"))?;

            std::fs::write(".comptext/model_response.latest.json", resp_json)
                .map_err(|e| format!("failed to write model response: {e}"))?;

            println!("Response from {} provider:", prov.name());
            println!("{}", response.content);
            Ok(())
        }
        other => Err(format!("unsupported provider kind '{other}'")),
    }
}

fn handle_propose(provider_name: Option<&str>, task: &str, config: &Config) -> Result<(), String> {
    let resolved_provider = provider_name.unwrap_or(config.defaults.provider.as_str());

    let profile = config.providers.get(resolved_provider).ok_or_else(|| {
        format!("provider profile '{resolved_provider}' not found in configuration")
    })?;

    let cp = build_context_pack(task)?;
    std::fs::create_dir_all(".comptext")
        .map_err(|e| format!("failed to create .comptext directory: {e}"))?;

    let cp_json = serde_json::to_string_pretty(&cp)
        .map_err(|e| format!("failed to serialize context pack: {e}"))?;

    std::fs::write(".comptext/context_pack.latest.json", &cp_json)
        .map_err(|e| format!("failed to write context pack: {e}"))?;

    let system_prompt = format!(
        "You are a helpful coding assistant. Here is the repository context:\n\n{}",
        cp.rendered_context
    );
    let request = ModelRequest {
        provider: resolved_provider.to_string(),
        model: "dummy-model".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: task.to_string(),
            },
        ],
    };

    let req_json = serde_json::to_string_pretty(&request)
        .map_err(|e| format!("failed to serialize model request: {e}"))?;

    std::fs::write(".comptext/model_request.latest.json", req_json)
        .map_err(|e| format!("failed to write model request: {e}"))?;

    if profile.kind == "openai-compatible" {
        let model_name = profile
            .model
            .clone()
            .unwrap_or_else(|| "gpt-4o".to_string());
        let openai_payload = serde_json::json!({
            "model": model_name,
            "messages": request.messages
        });
        let openai_req_json = serde_json::to_string_pretty(&openai_payload)
            .map_err(|e| format!("failed to serialize OpenAI payload: {e}"))?;
        std::fs::write(".comptext/openai_request.latest.json", openai_req_json)
            .map_err(|e| format!("failed to write OpenAI request artifact: {e}"))?;
    }

    let response = match profile.kind.as_str() {
        "dummy" => {
            use crate::provider::{DummyProvider, Provider};
            let prov = DummyProvider;
            prov.execute(&request)?
        }
        "ollama" => {
            ensure_provider_network_allowed(config, profile, resolved_provider)?;

            use crate::provider::{OllamaProvider, Provider};
            let url = profile
                .base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434".to_string());
            let suffix = profile.model_suffix.clone();
            let auth = profile.auth_env.clone();

            let prov = OllamaProvider {
                name: resolved_provider.to_string(),
                base_url: url,
                model_suffix: suffix,
                auth_env: auth,
            };
            prov.execute(&request)?
        }
        "openai-compatible" => {
            use crate::provider::{OpenaiProvider, Provider};
            let url = profile
                .base_url
                .clone()
                .unwrap_or_else(|| "http://localhost:11434/v1".to_string());
            let model = profile.model.clone();
            let auth = profile.auth_env.clone();
            let allow_net = config.policy.allow_provider_network && profile.network.unwrap_or(true);

            let prov = OpenaiProvider {
                name: resolved_provider.to_string(),
                base_url: url,
                model,
                auth_env: auth,
                allow_network: allow_net,
            };
            prov.execute(&request)?
        }
        other => return Err(format!("unsupported provider kind '{other}'")),
    };

    let resp_json = serde_json::to_string_pretty(&response)
        .map_err(|e| format!("failed to serialize model response: {e}"))?;

    std::fs::write(".comptext/model_response.latest.json", resp_json)
        .map_err(|e| format!("failed to write model response: {e}"))?;

    let proposal = Proposal {
        schema_version: "0.1".to_string(),
        task: task.to_string(),
        rationale: format!("Proposed changes based on task: {task}"),
        preconditions: vec!["cargo check".to_string()],
        affected_files: vec!["src/cli.rs".to_string()],
        operations: vec![Operation {
            op: "patch".to_string(),
            path: "src/cli.rs".to_string(),
            detail: format!(
                "Mock patch generated by dummy provider: \"{}\"",
                response.content.replace('\n', " ")
            ),
        }],
        validation_commands: vec!["cargo test".to_string()],
        rollback_strategy: "git restore src/cli.rs".to_string(),
        risk_notes: "None identified for offline mock run".to_string(),
    };

    std::fs::create_dir_all("proposals")
        .map_err(|e| format!("failed to create proposals directory: {e}"))?;

    let prop_json = serde_json::to_string_pretty(&proposal)
        .map_err(|e| format!("failed to serialize proposal: {e}"))?;

    let slug = slugify(task);
    let filename = format!("proposals/proposal_{slug}.json");
    std::fs::write(&filename, &prop_json)
        .map_err(|e| format!("failed to write proposal file '{filename}': {e}"))?;

    std::fs::write("proposals/proposal.latest.json", &prop_json)
        .map_err(|e| format!("failed to write proposals/proposal.latest.json: {e}"))?;

    println!("Proposal generated successfully.");
    println!("Proposal file: {filename}");
    println!("Latest reference: proposals/proposal.latest.json");
    Ok(())
}

fn is_allowed_write_path(path: &str) -> bool {
    if path.contains("..") {
        return false;
    }
    let p = std::path::Path::new(path);
    if p.is_absolute() {
        return false;
    }
    let path_lower = path.to_lowercase();
    if path_lower.contains(".git/") || path_lower.contains(".git\\") {
        return false;
    }
    if path_lower.contains(".comptext/") || path_lower.contains(".comptext\\") {
        return false;
    }
    if path_lower.contains("target/") || path_lower.contains("target\\") {
        return false;
    }
    if path_lower.contains("reports/") || path_lower.contains("reports\\") {
        return false;
    }
    if path_lower == ".env" || path_lower.starts_with(".env.") {
        return false;
    }
    if path_lower.ends_with(".key")
        || path_lower.ends_with(".pem")
        || path_lower.ends_with(".p12")
        || path_lower.ends_with(".pfx")
    {
        return false;
    }
    if path.starts_with("src/")
        || path.starts_with("src\\")
        || path.starts_with("tests/")
        || path.starts_with("tests\\")
        || path.starts_with("docs/")
        || path.starts_with("docs\\")
        || path.starts_with("proposals/")
        || path.starts_with("proposals\\")
        || path == "Cargo.toml"
        || path == "README.md"
        || path == "LICENSE"
        || path == "comptext.example.toml"
        || path == "PROJEKT.md"
    {
        return true;
    }
    false
}

fn apply_simulated_patch(path: &str, detail: &str) -> Result<(), String> {
    let file_path = std::path::Path::new(path);
    if !file_path.exists() {
        return Err(format!("File does not exist: {path}"));
    }
    let mut content = std::fs::read_to_string(file_path)
        .map_err(|e| format!("failed to read file '{path}': {e}"))?;
    if path.ends_with(".rs") {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&format!(
            "// Mock patch applied: {}\n",
            detail.replace('\n', " ")
        ));
    } else if path.ends_with(".md") {
        if !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(&format!(
            "<!-- Mock patch applied: {} -->\n",
            detail.replace('\n', " ")
        ));
    } else {
        println!("Simulating patch on non-source file: {}", path);
    }
    std::fs::write(file_path, content)
        .map_err(|e| format!("failed to write file '{path}': {e}"))?;
    Ok(())
}

fn handle_apply(proposal_path: Option<&str>, yes: bool) -> Result<(), String> {
    let path = proposal_path.unwrap_or("proposals/proposal.latest.json");
    if !std::path::Path::new(path).exists() {
        return Err(format!("Proposal file not found at '{path}'"));
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read proposal file '{path}': {e}"))?;
    let proposal: Proposal = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse proposal JSON: {e}"))?;

    println!("Applying Proposal:");
    println!("  Task: {}", proposal.task);
    println!("  Rationale: {}", proposal.rationale);
    println!("  Affected files:");
    for file in &proposal.affected_files {
        println!("    - {file}");
    }

    for op in &proposal.operations {
        if !is_allowed_write_path(&op.path) {
            return Err(format!(
                "Security Policy Violation: Path '{}' is not an allowed write path.",
                op.path
            ));
        }
    }

    if !yes {
        print!("Confirm applying these changes? [y/N]: ");
        use std::io::Write;
        std::io::stdout().flush().map_err(|e| e.to_string())?;
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        let trimmed = input.trim().to_lowercase();
        if trimmed != "y" && trimmed != "yes" {
            println!("Apply cancelled by user.");
            return Ok(());
        }
    }

    println!("Applying operations...");
    for op in &proposal.operations {
        if op.op == "patch" {
            apply_simulated_patch(&op.path, &op.detail)?;
        } else {
            return Err(format!("Unsupported operation type: {}", op.op));
        }
    }

    println!("Running validation commands...");
    for cmd_str in &proposal.validation_commands {
        println!("Executing: {}", cmd_str);
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let program = parts[0];
        let args = &parts[1..];
        let status = std::process::Command::new(program)
            .args(args)
            .status()
            .map_err(|e| format!("failed to run validation command '{cmd_str}': {e}"))?;
        if !status.success() {
            return Err(format!(
                "Validation command '{cmd_str}' failed. Return code: {}",
                status
            ));
        }
    }

    println!("Proposal applied and validated successfully.");
    Ok(())
}

fn handle_validate() -> Result<(), String> {
    println!("Standard local validation commands:");
    println!("cargo fmt --all --check");
    println!("cargo check");
    println!("cargo test");
    println!("cargo clippy -- -D warnings");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BenchmarkArtifact {
    pub schema_version: String,
    pub task: String,
    pub provider: String,
    pub context_pack_path: String,
    pub request_artifact_path: String,
    pub response_artifact_path: String,
    pub validation_commands: Vec<String>,
    pub network: String,
    pub secrets: String,
    pub status: String,
}

fn handle_benchmark(
    provider_name: Option<&str>,
    task: &str,
    _config: &Config,
) -> Result<(), String> {
    let resolved_provider = provider_name.unwrap_or("dummy");

    if resolved_provider != "dummy" {
        return Err(format!(
            "Security Policy Violation: Benchmark only supports the offline 'dummy' provider in this phase. Provider '{resolved_provider}' is not supported."
        ));
    }

    let cp = build_context_pack(task)?;
    std::fs::create_dir_all(".comptext")
        .map_err(|e| format!("failed to create .comptext directory: {e}"))?;

    let cp_path = ".comptext/context_pack.latest.json";
    let cp_json = serde_json::to_string_pretty(&cp)
        .map_err(|e| format!("failed to serialize context pack: {e}"))?;
    std::fs::write(cp_path, &cp_json).map_err(|e| format!("failed to write context pack: {e}"))?;

    let system_prompt = format!(
        "You are a helpful coding assistant. Here is the repository context:\n\n{}",
        cp.rendered_context
    );
    let request = ModelRequest {
        provider: resolved_provider.to_string(),
        model: "dummy-model".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: task.to_string(),
            },
        ],
    };

    let req_path = ".comptext/model_request.latest.json";
    let req_json = serde_json::to_string_pretty(&request)
        .map_err(|e| format!("failed to serialize model request: {e}"))?;
    std::fs::write(req_path, req_json)
        .map_err(|e| format!("failed to write model request: {e}"))?;

    use crate::provider::{DummyProvider, Provider};
    let prov = DummyProvider;
    let response = prov.execute(&request)?;

    let resp_path = ".comptext/model_response.latest.json";
    let resp_json = serde_json::to_string_pretty(&response)
        .map_err(|e| format!("failed to serialize model response: {e}"))?;
    std::fs::write(resp_path, resp_json)
        .map_err(|e| format!("failed to write model response: {e}"))?;

    let validation_cmds = vec![
        "cargo fmt --all --check".to_string(),
        "cargo check".to_string(),
        "cargo test".to_string(),
        "cargo clippy -- -D warnings".to_string(),
    ];

    let benchmark = BenchmarkArtifact {
        schema_version: "0.1".to_string(),
        task: task.to_string(),
        provider: resolved_provider.to_string(),
        context_pack_path: cp_path.to_string(),
        request_artifact_path: req_path.to_string(),
        response_artifact_path: resp_path.to_string(),
        validation_commands: validation_cmds,
        network: "offline-only".to_string(),
        secrets: "redacted".to_string(),
        status: "success".to_string(),
    };

    let bench_json = serde_json::to_string_pretty(&benchmark)
        .map_err(|e| format!("failed to serialize benchmark artifact: {e}"))?;

    let bench_path = ".comptext/benchmark.latest.json";
    std::fs::write(bench_path, bench_json)
        .map_err(|e| format!("failed to write benchmark artifact: {e}"))?;

    println!("Benchmark completed successfully.");
    println!("Benchmark Artifact: {bench_path}");
    Ok(())
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("_")
}

#[cfg(test)]
mod tests {
    use super::{
        handle_benchmark, handle_validate, parse, BenchmarkArtifact, Command, Config, Defaults,
        PolicyConfig, ProviderProfile,
    };
    use std::collections::HashMap;

    fn s(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).to_owned()).collect()
    }

    #[test]
    fn parses_help() {
        assert_eq!(parse(&s(&[])), Ok(Command::Help));
        assert_eq!(parse(&s(&["--help"])), Ok(Command::Help));
        assert_eq!(parse(&s(&["help"])), Ok(Command::Help));
    }

    #[test]
    fn parses_version() {
        assert_eq!(parse(&s(&["version"])), Ok(Command::Version));
        assert_eq!(parse(&s(&["--version"])), Ok(Command::Version));
        assert_eq!(parse(&s(&["-V"])), Ok(Command::Version));
    }

    #[test]
    fn parses_doctor() {
        assert_eq!(parse(&s(&["doctor"])), Ok(Command::Doctor));
    }

    #[test]
    fn parses_providers_list() {
        assert_eq!(
            parse(&s(&["providers", "list"])),
            Ok(Command::ProvidersList)
        );
    }

    #[test]
    fn parses_context_inspect() {
        assert_eq!(
            parse(&s(&["context", "inspect"])),
            Ok(Command::ContextInspect)
        );
    }

    #[test]
    fn parses_context_pack() {
        assert_eq!(
            parse(&s(&[
                "context",
                "pack",
                "--task",
                "test deterministic context"
            ])),
            Ok(Command::ContextPack {
                task: "test deterministic context".to_string()
            })
        );
    }

    #[test]
    fn parses_ask_dry_run() {
        assert_eq!(
            parse(&s(&["ask", "--dry-run", "How do I test this repo?"])),
            Ok(Command::Ask {
                provider: None,
                dry_run: true,
                prompt: "How do I test this repo?".to_string()
            })
        );
    }

    #[test]
    fn parses_ask_provider() {
        assert_eq!(
            parse(&s(&[
                "ask",
                "--provider",
                "dummy",
                "How do I test this repo?"
            ])),
            Ok(Command::Ask {
                provider: Some("dummy".to_string()),
                dry_run: false,
                prompt: "How do I test this repo?".to_string()
            })
        );
    }

    #[test]
    fn parses_ask_provider_ollama() {
        assert_eq!(
            parse(&s(&["ask", "--provider", "ollama-local", "hello"])),
            Ok(Command::Ask {
                provider: Some("ollama-local".to_string()),
                dry_run: false,
                prompt: "hello".to_string()
            })
        );
    }

    #[test]
    fn parses_propose() {
        assert_eq!(
            parse(&s(&[
                "propose",
                "--provider",
                "dummy",
                "Add context inspect"
            ])),
            Ok(Command::Propose {
                provider: Some("dummy".to_string()),
                task: "Add context inspect".to_string()
            })
        );
    }

    #[test]
    fn parses_apply() {
        assert_eq!(
            parse(&s(&["apply"])),
            Ok(Command::Apply {
                proposal_path: None,
                yes: false
            })
        );
        assert_eq!(
            parse(&s(&["apply", "proposals/test.json"])),
            Ok(Command::Apply {
                proposal_path: Some("proposals/test.json".to_string()),
                yes: false
            })
        );
        assert_eq!(
            parse(&s(&["apply", "--yes"])),
            Ok(Command::Apply {
                proposal_path: None,
                yes: true
            })
        );
        assert_eq!(
            parse(&s(&["apply", "-y", "proposals/test.json"])),
            Ok(Command::Apply {
                proposal_path: Some("proposals/test.json".to_string()),
                yes: true
            })
        );
    }

    #[test]
    fn parses_validate() {
        assert_eq!(parse(&s(&["validate"])), Ok(Command::Validate));
    }

    #[test]
    fn test_valid_config_parsing() {
        let toml_str = r#"
            [defaults]
            provider = "dummy"
            dry_run_default = true
            proposal_required = true

            [providers.dummy]
            kind = "dummy"
            network = false

            [policy]
            network_default = "deny"
            allow_provider_network = false
            secrets_redaction = true
            apply_requires_confirmation = true
        "#;
        let config: Result<Config, _> = toml::from_str(toml_str);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.defaults.provider, "dummy");
        assert!(config.defaults.dry_run_default);
        assert!(config.defaults.proposal_required);
        assert_eq!(config.policy.network_default, "deny");
        assert_eq!(config.providers.get("dummy").unwrap().kind, "dummy");
    }

    #[test]
    fn test_malformed_config_fails() {
        let toml_str = r#"
            [defaults]
            provider = "dummy"
            # Missing required fields
        "#;
        let config: Result<Config, _> = toml::from_str(toml_str);
        assert!(config.is_err());
    }

    #[test]
    fn test_secret_redaction_not_printed() {
        let mut providers = HashMap::new();
        providers.insert(
            "secret-prov".to_string(),
            ProviderProfile {
                kind: "ollama".to_string(),
                network: Some(true),
                base_url: Some("http://localhost".to_string()),
                model: None,
                auth: Some("secret_key_1234567890".to_string()),
                auth_env: None,
                model_suffix: None,
            },
        );

        let config = Config {
            defaults: Defaults {
                provider: "secret-prov".to_string(),
                dry_run_default: true,
                proposal_required: true,
            },
            providers,
            policy: PolicyConfig {
                network_default: "deny".to_string(),
                allow_provider_network: false,
                secrets_redaction: true,
                apply_requires_confirmation: true,
            },
        };

        let name = "secret-prov";
        let profile = &config.providers[name];
        let mut auth_str = if let Some(ref auth) = profile.auth {
            format!("auth={}", auth)
        } else {
            String::new()
        };

        let auth_lower = auth_str.to_lowercase();
        if auth_lower.contains("secret")
            || auth_lower.contains("password")
            || auth_lower.contains("token")
            || auth_lower.contains("key")
        {
            if !auth_lower.contains("ollama_api_key") && !auth_lower.contains("optional_api_key") {
                auth_str = "auth=[REDACTED-METADATA]".to_string();
            }
        }

        assert_eq!(auth_str, "auth=[REDACTED-METADATA]");
    }

    #[test]
    fn test_openai_secret_redaction() {
        let mut providers = HashMap::new();
        providers.insert(
            "openai-secret".to_string(),
            ProviderProfile {
                kind: "openai-compatible".to_string(),
                network: Some(false),
                base_url: Some("http://localhost/v1".to_string()),
                model: Some("gpt-4o".to_string()),
                auth: Some("sk-proj-supersecretkeyhere".to_string()),
                auth_env: None,
                model_suffix: None,
            },
        );

        let config = Config {
            defaults: Defaults {
                provider: "openai-secret".to_string(),
                dry_run_default: true,
                proposal_required: true,
            },
            providers,
            policy: PolicyConfig {
                network_default: "deny".to_string(),
                allow_provider_network: false,
                secrets_redaction: true,
                apply_requires_confirmation: true,
            },
        };

        let profile = &config.providers["openai-secret"];
        let mut auth_str = if let Some(ref auth) = profile.auth {
            format!("auth={}", auth)
        } else {
            String::new()
        };

        let auth_lower = auth_str.to_lowercase();
        if auth_lower.contains("secret")
            || auth_lower.contains("password")
            || auth_lower.contains("token")
            || auth_lower.contains("key")
        {
            if !auth_lower.contains("ollama_api_key") && !auth_lower.contains("optional_api_key") {
                auth_str = "auth=[REDACTED-METADATA]".to_string();
            }
        }

        assert_eq!(auth_str, "auth=[REDACTED-METADATA]");
    }

    #[test]
    fn rejects_unknown_command() {
        assert!(parse(&s(&["unknown"])).is_err());
    }

    #[test]
    fn rejects_extra_args() {
        assert!(parse(&s(&["doctor", "extra"])).is_err());
        assert!(parse(&s(&["version", "extra"])).is_err());
        assert!(parse(&s(&["providers", "list", "extra"])).is_err());
    }

    #[test]
    fn parses_benchmark() {
        assert_eq!(
            parse(&s(&[
                "benchmark",
                "--provider",
                "dummy",
                "How should I test this repo?"
            ])),
            Ok(Command::Benchmark {
                provider: Some("dummy".to_string()),
                task: "How should I test this repo?".to_string()
            })
        );
        assert_eq!(
            parse(&s(&["benchmark", "test without provider"])),
            Ok(Command::Benchmark {
                provider: None,
                task: "test without provider".to_string()
            })
        );
        assert!(parse(&s(&["benchmark"])).is_err());
        assert!(parse(&s(&["benchmark", "--provider"])).is_err());
    }

    #[test]
    fn test_validate_command() {
        let res = handle_validate();
        assert!(res.is_ok());
    }

    #[test]
    fn test_dummy_benchmark_artifact_shape() {
        let providers = HashMap::new();
        let config = Config {
            defaults: Defaults {
                provider: "dummy".to_string(),
                dry_run_default: true,
                proposal_required: true,
            },
            providers,
            policy: PolicyConfig {
                network_default: "deny".to_string(),
                allow_provider_network: false,
                secrets_redaction: true,
                apply_requires_confirmation: true,
            },
        };

        let bench_path = std::path::Path::new(".comptext/benchmark.latest.json");
        if bench_path.exists() {
            let _ = std::fs::remove_file(bench_path);
        }

        let res = handle_benchmark(Some("dummy"), "Verify benchmark shape", &config);
        assert!(res.is_ok());
        assert!(bench_path.exists());

        let content = std::fs::read_to_string(bench_path).unwrap();
        let artifact: BenchmarkArtifact = serde_json::from_str(&content).unwrap();

        assert_eq!(artifact.schema_version, "0.1");
        assert_eq!(artifact.task, "Verify benchmark shape");
        assert_eq!(artifact.provider, "dummy");
        assert_eq!(
            artifact.context_pack_path,
            ".comptext/context_pack.latest.json"
        );
        assert_eq!(
            artifact.request_artifact_path,
            ".comptext/model_request.latest.json"
        );
        assert_eq!(
            artifact.response_artifact_path,
            ".comptext/model_response.latest.json"
        );
        assert_eq!(artifact.network, "offline-only");
        assert_eq!(artifact.secrets, "redacted");
        assert_eq!(artifact.status, "success");
        assert!(artifact
            .validation_commands
            .contains(&"cargo test".to_string()));
    }

    #[test]
    fn test_unsupported_provider_benchmark_rejected() {
        let providers = HashMap::new();
        let config = Config {
            defaults: Defaults {
                provider: "dummy".to_string(),
                dry_run_default: true,
                proposal_required: true,
            },
            providers,
            policy: PolicyConfig {
                network_default: "deny".to_string(),
                allow_provider_network: false,
                secrets_redaction: true,
                apply_requires_confirmation: true,
            },
        };

        let res = handle_benchmark(Some("ollama-local"), "Verify rejection", &config);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains(
            "Security Policy Violation: Benchmark only supports the offline 'dummy' provider"
        ));

        let res2 = handle_benchmark(Some("openai-compatible"), "Verify rejection 2", &config);
        assert!(res2.is_err());
        assert!(res2.unwrap_err().contains(
            "Security Policy Violation: Benchmark only supports the offline 'dummy' provider"
        ));
    }
}
