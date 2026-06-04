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

#[test]
fn ask_dummy_provider_succeeds() {
    let stdout = run(&["ask", "--provider", "dummy", "How do I test this repo?"]);
    assert!(stdout.contains("Response from dummy provider:"));
    assert!(stdout.contains("Mock LLM response from CompText Dummy Provider."));
    assert!(stdout.contains("Received prompt: \"How do I test this repo?\""));

    // Verify response file was written
    let response_path = std::path::Path::new(".comptext/model_response.latest.json");
    assert!(response_path.exists());
    let response_content = std::fs::read_to_string(response_path).unwrap();
    assert!(response_content.contains("\"provider\": \"dummy\""));
}
