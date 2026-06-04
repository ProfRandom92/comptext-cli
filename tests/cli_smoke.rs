use std::process::Command;

fn run(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_ctxt"))
        .args(args)
        .output()
        .expect("ctxt binary should run");
    assert!(output.status.success(), "command failed: {args:?}");
    String::from_utf8(output.stdout).expect("stdout should be UTF-8")
}

#[test]
fn help_mentions_safety_defaults() {
    let stdout = run(&["--help"]);
    assert!(stdout.contains("SAFETY DEFAULTS"));
    assert!(stdout.contains("network_default=deny"));
}

#[test]
fn doctor_is_local_and_deterministic() {
    let stdout = run(&["doctor"]);
    assert!(stdout.contains("status: ok"));
    assert!(stdout.contains("provider_default: dummy"));
}

#[test]
fn providers_include_dummy_and_ollama_variants() {
    let stdout = run(&["providers", "list"]);
    assert!(stdout.contains("dummy"));
    assert!(stdout.contains("ollama-local"));
    assert!(stdout.contains("ollama-cloud-direct"));
}
