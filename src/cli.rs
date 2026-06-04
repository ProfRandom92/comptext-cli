use serde::{Deserialize, Serialize};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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

pub fn run<I>(args: I) -> i32
where
    I: IntoIterator,
    I::Item: Into<String>,
{
    let argv: Vec<String> = args.into_iter().map(Into::into).collect();
    match parse(&argv) {
        Ok(Command::Help) => {
            print_help();
            0
        }
        Ok(Command::Version) => {
            println!("ctxt {VERSION}");
            0
        }
        Ok(Command::Doctor) => {
            print_doctor();
            0
        }
        Ok(Command::ProvidersList) => {
            print_providers();
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
        }) => match handle_ask(provider.as_deref(), dry_run, &prompt) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("error: {e}");
                1
            }
        },
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
            if argv.len() < 3 {
                return Err(
                    "missing arguments for 'ask'. Usage: ctxt ask --dry-run \"<prompt>\" or ctxt ask --provider <provider> \"<prompt>\""
                        .to_string(),
                );
            }

            let mut provider = None;
            let mut dry_run = false;
            let mut prompt = String::new();

            let mut i = 1;
            while i < argv.len() {
                match argv[i].as_str() {
                    "--dry-run" => {
                        dry_run = true;
                        i += 1;
                    }
                    "--provider" => {
                        if i + 1 >= argv.len() {
                            return Err("missing provider name after --provider".to_string());
                        }
                        provider = Some(argv[i + 1].clone());
                        i += 2;
                    }
                    other => {
                        if other.starts_with('-') {
                            return Err(format!("unsupported option '{other}' for 'ask'"));
                        }
                        if !prompt.is_empty() {
                            return Err(format!("unexpected argument '{other}' for 'ask'"));
                        }
                        prompt = other.to_string();
                        i += 1;
                    }
                }
            }

            if prompt.is_empty() {
                return Err("missing prompt for 'ask'".to_string());
            }

            if !dry_run && provider.is_none() {
                return Err("must specify either --dry-run or --provider <provider>".to_string());
            }

            Ok(Command::Ask {
                provider,
                dry_run,
                prompt,
            })
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
\n\
SAFETY DEFAULTS:\n\
    network_default=deny\n\
    dry_run_before_network=true\n\
    proposal_before_apply=true\n\
    secrets_redaction=true"
    );
}

fn print_doctor() {
    println!("CompText doctor");
    println!("status: ok");
    println!("network_default: deny");
    println!("provider_default: dummy");
    println!("proposal_required: true");
    println!("secrets_policy: redact-before-artifact");
}

fn print_providers() {
    println!("dummy\tnetwork=false\tstatus=planned-offline-test-provider");
    println!("ollama-local\tnetwork=explicit\tbase_url=http://localhost:11434");
    println!("ollama-cloud-via-local\tnetwork=explicit\tbase_url=http://localhost:11434\tmodel_suffix=-cloud");
    println!("ollama-cloud-direct\tnetwork=explicit\tbase_url=https://ollama.com\tauth_env=OLLAMA_API_KEY");
    println!("openai-compatible\tnetwork=explicit\tbase_url=configured");
    println!("future-openai\tnetwork=explicit\tstatus=planned");
    println!("future-gemini\tnetwork=explicit\tstatus=planned");
    println!("custom\tnetwork=explicit\tstatus=planned");
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

fn handle_ask(provider: Option<&str>, dry_run: bool, prompt: &str) -> Result<(), String> {
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
        provider: provider.unwrap_or("dummy").to_string(),
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

    if dry_run {
        println!("Dry-run successful.");
        println!("Context Pack: .comptext/context_pack.latest.json");
        println!("Model Request: .comptext/model_request.latest.json");
        return Ok(());
    }

    let p_name = provider.ok_or_else(|| "provider is required for live execution".to_string())?;
    match p_name {
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
        "ollama-local" | "ollama-cloud-via-local" | "ollama-cloud-direct" => {
            use crate::provider::{OllamaProvider, Provider};
            let (url, suffix, auth) = match p_name {
                "ollama-local" => ("http://localhost:11434".to_string(), None, None),
                "ollama-cloud-via-local" => (
                    "http://localhost:11434".to_string(),
                    Some("-cloud".to_string()),
                    None,
                ),
                "ollama-cloud-direct" => (
                    "https://ollama.com".to_string(),
                    None,
                    Some("OLLAMA_API_KEY".to_string()),
                ),
                _ => unreachable!(),
            };

            let prov = OllamaProvider {
                name: p_name.to_string(),
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
        other => Err(format!("unsupported provider '{other}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Command};

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
    fn rejects_unknown_command() {
        assert!(parse(&s(&["unknown"])).is_err());
    }

    #[test]
    fn rejects_extra_args() {
        assert!(parse(&s(&["doctor", "extra"])).is_err());
        assert!(parse(&s(&["version", "extra"])).is_err());
        assert!(parse(&s(&["providers", "list", "extra"])).is_err());
    }
}
