use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;
use tempfile::tempdir;

fn crg() -> Command {
    Command::cargo_bin("crg").unwrap()
}

#[test]
fn documented_init_stamp_check_flow_passes() {
    let directory = tempdir().unwrap();
    let config = directory.path().join("agent.json");
    fs::write(&config, r#"{"permissions":{"shell":false},"retries":2}"#).unwrap();

    crg()
        .args(["init", config.to_str().unwrap()])
        .assert()
        .success();
    let sidecar = directory.path().join("agent.json.rationale.json");
    let mut rationale: Value =
        serde_json::from_str(&fs::read_to_string(&sidecar).unwrap()).unwrap();
    for decision in rationale["decisions"].as_array_mut().unwrap() {
        decision["rationale"] = Value::String("Reviewed for the documented test flow.".into());
    }
    rationale["rules"] = serde_json::json!([{
        "pattern": "/permissions/**",
        "minimumCoverage": 1.0
    }]);
    fs::write(&sidecar, serde_json::to_string_pretty(&rationale).unwrap()).unwrap();

    crg()
        .args(["stamp", config.to_str().unwrap()])
        .assert()
        .success();
    crg()
        .args(["check", config.to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"valid\": true"));
}

#[test]
fn stale_rationale_fails_without_printing_the_value() {
    let directory = tempdir().unwrap();
    let config = directory.path().join("ci.yaml");
    fs::write(&config, "token: first-secret\n").unwrap();
    crg()
        .args(["init", config.to_str().unwrap()])
        .assert()
        .success();
    let sidecar = directory.path().join("ci.yaml.rationale.json");
    let mut rationale: Value =
        serde_json::from_str(&fs::read_to_string(&sidecar).unwrap()).unwrap();
    rationale["decisions"][0]["rationale"] =
        Value::String("Required by the deployment environment.".into());
    fs::write(&sidecar, serde_json::to_string_pretty(&rationale).unwrap()).unwrap();
    fs::write(&config, "token: second-secret\n").unwrap();

    crg()
        .args(["check", config.to_str().unwrap()])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("stale_rationale"))
        .stdout(predicate::str::contains("second-secret").not());
}

#[test]
fn schema_and_duplicate_json_are_rejected() {
    let directory = tempdir().unwrap();
    let duplicate = directory.path().join("bad.json");
    fs::write(&duplicate, r#"{"mode":"a","mode":"b"}"#).unwrap();
    crg()
        .args(["init", duplicate.to_str().unwrap()])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("duplicate object key"));

    let config = directory.path().join("agent.toml");
    let schema = directory.path().join("schema.json");
    fs::write(&config, "retries = 9\n").unwrap();
    fs::write(
        &schema,
        r#"{"type":"object","properties":{"retries":{"maximum":3}}}"#,
    )
    .unwrap();
    crg()
        .args(["init", config.to_str().unwrap()])
        .assert()
        .success();
    crg()
        .args([
            "check",
            config.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("schema_violation"));
}

#[test]
fn schema_reports_redact_the_failing_value_in_human_and_json_output() {
    const SECRET: &str = "QA_SCHEMA_SECRET_DO_NOT_REPORT_7c75506b";
    let directory = tempdir().unwrap();
    let config = directory.path().join("secrets.json");
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

    for output in [false, true] {
        let mut command = crg();
        command.args([
            "check",
            config.to_str().unwrap(),
            "--schema",
            schema.to_str().unwrap(),
        ]);
        if output {
            command.arg("--json");
        }
        command
            .assert()
            .code(1)
            .stdout(predicate::str::contains("schema_violation"))
            .stdout(predicate::str::contains(
                "configuration does not satisfy the supplied JSON Schema",
            ))
            .stdout(predicate::str::contains(SECRET).not());
    }
}

#[test]
fn diff_reports_paths_and_rationale_but_not_values() {
    let directory = tempdir().unwrap();
    let base = directory.path().join("base.json");
    let head = directory.path().join("head.json");
    fs::write(&base, r#"{"sandbox":"old-private-value"}"#).unwrap();
    fs::write(&head, r#"{"sandbox":"new-private-value"}"#).unwrap();
    crg()
        .args(["init", base.to_str().unwrap()])
        .assert()
        .success();
    crg()
        .args(["init", head.to_str().unwrap()])
        .assert()
        .success();
    let sidecar = directory.path().join("head.json.rationale.json");
    let mut rationale: Value =
        serde_json::from_str(&fs::read_to_string(&sidecar).unwrap()).unwrap();
    rationale["decisions"][0]["rationale"] =
        Value::String("Isolation mode changed for signed release jobs.".into());
    fs::write(&sidecar, serde_json::to_string_pretty(&rationale).unwrap()).unwrap();
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
        .stdout(predicate::str::contains("Isolation mode changed"))
        .stdout(predicate::str::contains("new-private-value").not())
        .stdout(predicate::str::contains("old-private-value").not());
}
