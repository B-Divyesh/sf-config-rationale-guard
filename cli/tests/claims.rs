use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{Value, json};
use std::fs;
use tempfile::tempdir;

fn crg() -> Command {
    Command::cargo_bin("crg").unwrap()
}

fn sample_project() -> (tempfile::TempDir, std::path::PathBuf) {
    let directory = tempdir().unwrap();
    let config = directory.path().join("agent.json");
    crg()
        .args(["demo", "--output", directory.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("[PASS]"));
    (directory, config)
}

/// @claim:cli-demo
#[test]
fn cli_demo_creates_a_real_passing_sample_project() {
    let (directory, config) = sample_project();
    assert!(config.exists());
    assert!(directory.path().join("agent.json.rationale.json").exists());
    assert!(directory.path().join("agent.schema.json").exists());
    crg()
        .args([
            "check",
            config.to_str().unwrap(),
            "--schema",
            directory.path().join("agent.schema.json").to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"));
}

/// @claim:supported-formats
#[test]
fn json_yaml_and_toml_complete_the_same_rationale_flow() {
    let directory = tempdir().unwrap();
    let schema = directory.path().join("agent.schema.json");
    fs::write(
        &schema,
        r#"{"type":"object","properties":{"retries":{"type":"integer","minimum":0,"maximum":3}}}"#,
    )
    .unwrap();
    let formats = [
        ("agent.json", r#"{"retries":2}"#),
        ("agent.yaml", "retries: 2\n"),
        ("agent.toml", "retries = 2\n"),
    ];
    for (name, source) in formats {
        let config = directory.path().join(name);
        fs::write(&config, source).unwrap();
        crg()
            .args(["init", config.to_str().unwrap()])
            .assert()
            .success();
        let sidecar = directory.path().join(format!("{name}.rationale.json"));
        let mut rationale: Value =
            serde_json::from_str(&fs::read_to_string(&sidecar).unwrap()).unwrap();
        rationale["decisions"][0]["rationale"] = json!("Reviewed in the format support test.");
        fs::write(&sidecar, serde_json::to_string_pretty(&rationale).unwrap()).unwrap();
        crg()
            .args(["stamp", config.to_str().unwrap()])
            .assert()
            .success();
        crg()
            .args([
                "check",
                config.to_str().unwrap(),
                "--schema",
                schema.to_str().unwrap(),
                "--json",
            ])
            .assert()
            .success()
            .stdout(predicate::str::contains("\"valid\": true"));
    }
}

/// @claim:adjacent-rationale
#[test]
fn init_writes_an_adjacent_rationale_file_for_each_setting() {
    let directory = tempdir().unwrap();
    let config = directory.path().join("ci.json");
    fs::write(&config, r#"{"retries":2,"permissions":{"shell":false}}"#).unwrap();
    crg()
        .args(["init", config.to_str().unwrap()])
        .assert()
        .success();
    let sidecar = directory.path().join("ci.json.rationale.json");
    let created: Value = serde_json::from_str(&fs::read_to_string(sidecar).unwrap()).unwrap();
    let paths: Vec<_> = created["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|decision| decision["path"].as_str())
        .collect();
    assert_eq!(paths, ["/permissions/shell", "/retries"]);
}

/// @claim:source-preserved
#[test]
fn init_and_stamp_leave_the_source_config_byte_for_byte_unchanged() {
    let directory = tempdir().unwrap();
    let config = directory.path().join("agent.yaml");
    let source = "# Keep the review comment\nretries: 2  # maintain release speed\n";
    fs::write(&config, source).unwrap();
    crg()
        .args(["init", config.to_str().unwrap()])
        .assert()
        .success();
    let sidecar = directory.path().join("agent.yaml.rationale.json");
    let mut rationale: Value =
        serde_json::from_str(&fs::read_to_string(&sidecar).unwrap()).unwrap();
    rationale["decisions"][0]["rationale"] = json!("Two retries limit duplicate release work.");
    fs::write(&sidecar, serde_json::to_string_pretty(&rationale).unwrap()).unwrap();
    crg()
        .args(["stamp", config.to_str().unwrap()])
        .assert()
        .success();
    assert_eq!(fs::read_to_string(config).unwrap(), source);
}

/// @claim:schema-validation
#[test]
fn schema_validation_finds_an_invalid_normalized_config() {
    let (directory, config) = sample_project();
    fs::write(
        &config,
        r#"{"permissions":{"shell":false,"network":"allowlist"},"retries":9}"#,
    )
    .unwrap();
    crg()
        .args([
            "check",
            config.to_str().unwrap(),
            "--schema",
            directory.path().join("agent.schema.json").to_str().unwrap(),
            "--json",
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("schema_violation"))
        .stdout(predicate::str::contains("/retries"));
}

/// @claim:decision-findings
#[test]
fn check_reports_missing_orphaned_overdue_and_stale_decisions() {
    let directory = tempdir().unwrap();
    let config = directory.path().join("agent.json");
    let sidecar = directory.path().join("agent.json.rationale.json");
    fs::write(&config, r#"{"present":"current"}"#).unwrap();
    fs::write(
        &sidecar,
        serde_json::to_string_pretty(&json!({
            "version": 1,
            "decisions": [
                {"path":"/present","rationale":"TODO","reviewBy":"2020-01-01","valueHash":"sha256:old"},
                {"path":"/removed","rationale":"This setting was removed from the configuration.","valueHash":"sha256:old"}
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    crg()
        .args(["check", config.to_str().unwrap(), "--json"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("missing_rationale"))
        .stdout(predicate::str::contains("stale_rationale"))
        .stdout(predicate::str::contains("review_overdue"))
        .stdout(predicate::str::contains("orphaned_rationale"));
}

/// @claim:value-free-reports
#[test]
fn human_and_json_reports_do_not_print_config_values() {
    const SECRET: &str = "claim-secret-0dc5e8b3-95ec-45e7-9ea5-1f342d543e60";
    let directory = tempdir().unwrap();
    let config = directory.path().join("secret.json");
    let schema = directory.path().join("schema.json");
    fs::write(&config, format!(r#"{{"token":"{SECRET}"}}"#)).unwrap();
    fs::write(
        &schema,
        r#"{"type":"object","properties":{"token":{"type":"integer"}}}"#,
    )
    .unwrap();
    crg()
        .args(["init", config.to_str().unwrap()])
        .assert()
        .success();
    for json in [false, true] {
        let mut command = crg();
        command.args([
            "check",
            config.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ]);
        if json {
            command.arg("--json");
        }
        command
            .assert()
            .code(1)
            .stdout(predicate::str::contains("schema_violation"))
            .stdout(predicate::str::contains(SECRET).not());
    }

    let base = directory.path().join("base.json");
    let head = directory.path().join("head.json");
    fs::write(&base, r#"{"token":"before-secret-claim"}"#).unwrap();
    fs::write(&head, r#"{"token":"after-secret-claim"}"#).unwrap();
    crg()
        .args(["init", base.to_str().unwrap()])
        .assert()
        .success();
    crg()
        .args(["init", head.to_str().unwrap()])
        .assert()
        .success();
    let head_sidecar = directory.path().join("head.json.rationale.json");
    let mut rationale: Value =
        serde_json::from_str(&fs::read_to_string(&head_sidecar).unwrap()).unwrap();
    rationale["decisions"][0]["rationale"] =
        json!("The token rotated during the release exercise.");
    fs::write(
        &head_sidecar,
        serde_json::to_string_pretty(&rationale).unwrap(),
    )
    .unwrap();
    crg()
        .args(["stamp", head.to_str().unwrap()])
        .assert()
        .success();
    crg()
        .args([
            "diff",
            base.to_str().unwrap(),
            head.to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "The token rotated during the release exercise.",
        ))
        .stdout(predicate::str::contains("before-secret-claim").not())
        .stdout(predicate::str::contains("after-secret-claim").not());
}

/// @claim:json-output
#[test]
fn check_json_output_is_a_single_machine_readable_report() {
    let (directory, config) = sample_project();
    let output = crg()
        .args([
            "check",
            config.to_str().unwrap(),
            "--schema",
            directory.path().join("agent.schema.json").to_str().unwrap(),
            "--json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let report: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(report["valid"], true);
    assert!(report["coverage"].as_array().is_some());
}

/// @claim:exit-codes
#[test]
fn check_uses_documented_success_finding_and_input_exit_codes() {
    let (directory, config) = sample_project();
    crg()
        .args(["check", config.to_str().unwrap()])
        .assert()
        .success();
    fs::write(
        &config,
        r#"{"permissions":{"shell":false,"network":"allowlist"},"retries":99}"#,
    )
    .unwrap();
    crg()
        .args(["check", config.to_str().unwrap()])
        .assert()
        .code(1);
    crg()
        .args([
            "check",
            directory.path().join("missing.json").to_str().unwrap(),
        ])
        .assert()
        .code(2);
}

/// @claim:local-cli
#[test]
fn cli_demo_runs_in_a_fresh_directory_without_setup_or_network_input() {
    let directory = tempdir().unwrap();
    crg()
        .args(["demo", "--output", directory.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Sample project created at"))
        .stdout(predicate::str::contains("[PASS]"));
}

/// @claim:mit-license
#[test]
fn distributed_cli_declares_the_mit_license() {
    assert_eq!(env!("CARGO_PKG_LICENSE"), "MIT");
}
