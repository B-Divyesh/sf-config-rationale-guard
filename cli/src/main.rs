use clap::{Args, Parser, Subcommand};
use config_rationale_guard::{
    config::{leaf_paths, load, pattern_matches, pointer},
    guard::{error, fingerprint, has_rationale, validate},
    model::{
        CheckReport, Decision, DiffEntry, DiffReport, DiffSummary, Finding, RationaleFile, Severity,
    },
};
use serde::Serialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    name = "crg",
    version,
    about = "Keep configuration rationale visible, current, and reviewable",
    long_about = "Config Rationale Guard pairs strict JSON, YAML, or TOML with an adjacent rationale sidecar. It validates decision targets and fingerprints without printing config values or sending data over the network."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a sidecar with one TODO decision per config leaf
    Init(InitArgs),
    /// Fingerprint reviewed decisions after their rationale has been updated
    Stamp(CommonArgs),
    /// Validate config, schema, rationale targets, freshness, and coverage
    Check(CheckArgs),
    /// Render a value-free report of changed settings and decisions
    Diff(DiffArgs),
}

#[derive(Args)]
struct InitArgs {
    /// JSON, YAML, or TOML configuration file
    config: PathBuf,
    /// Write the sidecar here instead of <config>.rationale.json
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Replace an existing sidecar
    #[arg(long)]
    force: bool,
    /// Emit a machine-readable result
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct CommonArgs {
    /// JSON, YAML, or TOML configuration file
    config: PathBuf,
    /// Read this sidecar instead of <config>.rationale.json
    #[arg(short, long)]
    rationales: Option<PathBuf>,
    /// Emit a machine-readable result
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct CheckArgs {
    #[command(flatten)]
    common: CommonArgs,
    /// Validate normalized config with this JSON Schema
    #[arg(short, long)]
    schema: Option<PathBuf>,
}

#[derive(Args)]
struct DiffArgs {
    /// Configuration before the change
    base: PathBuf,
    /// Configuration after the change
    head: PathBuf,
    /// Base sidecar (defaults to <base>.rationale.json)
    #[arg(long)]
    base_rationales: Option<PathBuf>,
    /// Head sidecar (defaults to <head>.rationale.json)
    #[arg(long)]
    head_rationales: Option<PathBuf>,
    /// Validate the head config with this JSON Schema
    #[arg(short, long)]
    schema: Option<PathBuf>,
    /// Emit a machine-readable result
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(valid) if valid => ExitCode::SUCCESS,
        Ok(_) => ExitCode::from(1),
        Err(message) => {
            eprintln!("crg: {message}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool, String> {
    match cli.command {
        Command::Init(args) => init(args),
        Command::Stamp(args) => stamp(args),
        Command::Check(args) => check(args),
        Command::Diff(args) => diff(args),
    }
}

fn sidecar_path(config: &Path) -> PathBuf {
    let mut name = config.as_os_str().to_owned();
    name.push(".rationale.json");
    PathBuf::from(name)
}

fn init(args: InitArgs) -> Result<bool, String> {
    let (config, _) = load(&args.config)?;
    let output = args.output.unwrap_or_else(|| sidecar_path(&args.config));
    if output.exists() && !args.force {
        return Err(format!(
            "{} already exists; pass --force to replace it",
            output.display()
        ));
    }
    let decisions = leaf_paths(&config)
        .into_iter()
        .map(|path| Decision {
            value_hash: fingerprint(pointer(&config, &path).expect("leaf exists")),
            path,
            rationale: "TODO".into(),
            policy: None,
            owner: None,
            review_by: None,
        })
        .collect::<Vec<_>>();
    let count = decisions.len();
    let rationale = RationaleFile {
        version: 1,
        rules: Vec::new(),
        decisions,
    };
    write_json(&output, &rationale)?;
    if args.json {
        print_json(&serde_json::json!({
            "created": output,
            "decisions": count,
            "next": "Replace TODO rationales, then run crg stamp"
        }))?;
    } else {
        println!("Created {} with {count} decision(s).", output.display());
        println!(
            "Next: replace TODO rationales, then run `crg stamp {}`.",
            args.config.display()
        );
    }
    Ok(true)
}

fn stamp(args: CommonArgs) -> Result<bool, String> {
    let (config, _) = load(&args.config)?;
    let path = args
        .rationales
        .unwrap_or_else(|| sidecar_path(&args.config));
    let mut rationale = load_rationale(&path)?;
    let mut stamped = 0;
    let mut skipped = 0;
    for decision in &mut rationale.decisions {
        if !has_rationale(decision) {
            skipped += 1;
            continue;
        }
        let Some(value) = pointer(&config, &decision.path) else {
            skipped += 1;
            continue;
        };
        decision.value_hash = fingerprint(value);
        stamped += 1;
    }
    write_json(&path, &rationale)?;
    if args.json {
        print_json(
            &serde_json::json!({ "updated": path, "stamped": stamped, "skipped": skipped }),
        )?;
    } else {
        println!(
            "Stamped {stamped} reviewed decision(s) in {}.",
            path.display()
        );
        if skipped > 0 {
            println!("Skipped {skipped} TODO or orphaned decision(s).");
        }
    }
    Ok(skipped == 0)
}

fn check(args: CheckArgs) -> Result<bool, String> {
    let (config, format) = load(&args.common.config)?;
    let sidecar = args
        .common
        .rationales
        .unwrap_or_else(|| sidecar_path(&args.common.config));
    let rationale = load_rationale(&sidecar)?;
    let (mut findings, coverage) = validate(&config, &rationale);
    let schema_validated = args.schema.is_some();
    if let Some(schema) = &args.schema {
        findings.extend(validate_schema(&config, schema)?);
    }
    let valid = !findings
        .iter()
        .any(|finding| matches!(finding.severity, Severity::Error));
    let report = CheckReport {
        valid,
        config: args.common.config.display().to_string(),
        rationale_file: sidecar.display().to_string(),
        format: format.into(),
        schema_validated,
        decisions: rationale.decisions.len(),
        coverage,
        findings,
    };
    if args.common.json {
        print_json(&report)?;
    } else {
        print_check(&report);
    }
    Ok(valid)
}

fn diff(args: DiffArgs) -> Result<bool, String> {
    let (base, _) = load(&args.base)?;
    let (head, _) = load(&args.head)?;
    let base_path = args
        .base_rationales
        .unwrap_or_else(|| sidecar_path(&args.base));
    let head_path = args
        .head_rationales
        .unwrap_or_else(|| sidecar_path(&args.head));
    let base_rationale = load_rationale_or_empty(&base_path)?;
    let head_rationale = load_rationale_or_empty(&head_path)?;
    let mut findings = validate(&head, &head_rationale).0;
    if let Some(schema) = &args.schema {
        findings.extend(validate_schema(&head, schema)?);
    }

    let base_leaves = values_by_path(&base);
    let head_leaves = values_by_path(&head);
    let all_paths: BTreeSet<_> = base_leaves
        .keys()
        .chain(head_leaves.keys())
        .cloned()
        .collect();
    let base_decisions = decisions_by_path(&base_rationale);
    let head_decisions = decisions_by_path(&head_rationale);
    let mut changes = Vec::new();
    for path in all_paths {
        let before = base_leaves.get(&path);
        let after = head_leaves.get(&path);
        if before == after {
            continue;
        }
        let setting_change = match (before, after) {
            (None, Some(_)) => "added",
            (Some(_), None) => "removed",
            _ => "changed",
        };
        let old_decision = base_decisions.get(path.as_str()).copied();
        let new_decision = head_decisions.get(path.as_str()).copied();
        let rationale_change = match (old_decision, new_decision) {
            (None, Some(_)) => "added",
            (Some(_), None) => "removed",
            (Some(old), Some(new)) if old.rationale != new.rationale => "changed",
            _ => "unchanged",
        };
        let required = head_rationale
            .rules
            .iter()
            .any(|rule| pattern_matches(&rule.pattern, &path));
        let status = match (after, new_decision) {
            (Some(value), Some(decision)) if decision.value_hash != fingerprint(value) => "stale",
            (_, Some(decision)) if !has_rationale(decision) => "needs rationale",
            (_, None) if required => "uncovered",
            _ => "ready",
        };
        let decision = new_decision.or(old_decision);
        changes.push(DiffEntry {
            path,
            setting_change,
            rationale_change,
            rationale: decision.map(|item| item.rationale.clone()),
            policy: decision.and_then(|item| item.policy.clone()),
            owner: decision.and_then(|item| item.owner.clone()),
            review_by: decision.and_then(|item| item.review_by.clone()),
            status,
        });
    }
    let rationales_changed = changes
        .iter()
        .filter(|change| change.rationale_change != "unchanged")
        .count();
    let attention_required = changes
        .iter()
        .filter(|change| change.status != "ready")
        .count();
    let valid = attention_required == 0
        && !findings
            .iter()
            .any(|finding| matches!(finding.severity, Severity::Error));
    let report = DiffReport {
        valid,
        base: args.base.display().to_string(),
        head: args.head.display().to_string(),
        summary: DiffSummary {
            settings_changed: changes.len(),
            rationales_changed,
            attention_required,
        },
        changes,
        findings,
    };
    if args.json {
        print_json(&report)?;
    } else {
        print_diff(&report);
    }
    Ok(valid)
}

fn load_rationale(path: &Path) -> Result<RationaleFile, String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("could not read rationale file {}: {error}", path.display()))?;
    serde_json::from_str(&source)
        .map_err(|error| format!("invalid rationale file {}: {error}", path.display()))
}

fn load_rationale_or_empty(path: &Path) -> Result<RationaleFile, String> {
    if path.exists() {
        load_rationale(path)
    } else {
        Ok(RationaleFile::default())
    }
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let output = serde_json::to_string_pretty(value)
        .map_err(|error| format!("could not encode {}: {error}", path.display()))?;
    fs::write(path, format!("{output}\n"))
        .map_err(|error| format!("could not write {}: {error}", path.display()))
}

fn print_json(value: &impl Serialize) -> Result<(), String> {
    let output = serde_json::to_string_pretty(value)
        .map_err(|error| format!("could not encode JSON output: {error}"))?;
    println!("{output}");
    Ok(())
}

fn validate_schema(config: &Value, path: &Path) -> Result<Vec<Finding>, String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("could not read schema {}: {error}", path.display()))?;
    let schema: Value = serde_json::from_str(&source)
        .map_err(|error| format!("invalid JSON Schema {}: {error}", path.display()))?;
    let validator = jsonschema::validator_for(&schema)
        .map_err(|error| format!("invalid JSON Schema {}: {error}", path.display()))?;
    Ok(validator
        .iter_errors(config)
        .map(|issue| {
            error(
                "schema_violation",
                &issue.instance_path.to_string(),
                "configuration does not satisfy the supplied JSON Schema".into(),
            )
        })
        .collect())
}

fn values_by_path(value: &Value) -> BTreeMap<String, &Value> {
    leaf_paths(value)
        .into_iter()
        .map(|path| {
            let value = pointer(value, &path).expect("leaf path");
            (path, value)
        })
        .collect()
}

fn decisions_by_path(rationale: &RationaleFile) -> BTreeMap<&str, &Decision> {
    rationale
        .decisions
        .iter()
        .map(|decision| (decision.path.as_str(), decision))
        .collect()
}

fn print_check(report: &CheckReport) {
    let mark = if report.valid { "PASS" } else { "FAIL" };
    println!(
        "[{mark}] {} · {} · {} decision(s)",
        report.config, report.format, report.decisions
    );
    for coverage in &report.coverage {
        println!(
            "  coverage {}: {}/{} ({:.0}%, minimum {:.0}%)",
            coverage.pattern,
            coverage.covered,
            coverage.total,
            coverage.ratio * 100.0,
            coverage.minimum * 100.0
        );
    }
    for finding in &report.findings {
        println!(
            "  {} {} {} — {}",
            severity(finding),
            finding.code,
            display_path(&finding.path),
            finding.message
        );
    }
    if report.findings.is_empty() {
        println!("  No stale, orphaned, uncovered, or schema-invalid decisions.");
    }
}

fn print_diff(report: &DiffReport) {
    println!("Decision diff · {} → {}", report.base, report.head);
    println!(
        "{} setting(s) changed · {} rationale change(s) · {} need attention",
        report.summary.settings_changed,
        report.summary.rationales_changed,
        report.summary.attention_required
    );
    if report.changes.is_empty() {
        println!("  No configuration decisions changed.");
    }
    for change in &report.changes {
        println!(
            "\n{}  {} · rationale {} · {}",
            change.path, change.setting_change, change.rationale_change, change.status
        );
        if let Some(rationale) = &change.rationale {
            println!("  why: {rationale}");
        }
        let metadata = [
            change.policy.as_deref().map(|v| format!("policy {v}")),
            change.owner.as_deref().map(|v| format!("owner {v}")),
            change
                .review_by
                .as_deref()
                .map(|v| format!("review by {v}")),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        if !metadata.is_empty() {
            println!("  {}", metadata.join(" · "));
        }
    }
    if !report.findings.is_empty() {
        println!("\nFindings");
        for finding in &report.findings {
            println!(
                "  {} {} {} — {}",
                severity(finding),
                finding.code,
                display_path(&finding.path),
                finding.message
            );
        }
    }
}

fn severity(finding: &Finding) -> &'static str {
    match finding.severity {
        Severity::Error => "ERROR",
        Severity::Warning => "WARN ",
    }
}

fn display_path(path: &str) -> &str {
    if path.is_empty() { "/" } else { path }
}
