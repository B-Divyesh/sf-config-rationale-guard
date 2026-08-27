use crate::{
    config::{leaf_paths, pattern_matches, pointer},
    model::{Coverage, Decision, Finding, RationaleFile, Severity},
};
use chrono::{NaiveDate, Utc};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub fn fingerprint(value: &Value) -> String {
    let canonical = canonicalize(value);
    let bytes = serde_json::to_vec(&canonical).expect("JSON values always serialize");
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn canonicalize(value: &Value) -> Value {
    match value {
        Value::Object(object) => {
            let sorted: BTreeMap<_, _> = object
                .iter()
                .map(|(key, value)| (key.clone(), canonicalize(value)))
                .collect();
            serde_json::to_value(sorted).expect("canonical JSON")
        }
        Value::Array(values) => Value::Array(values.iter().map(canonicalize).collect()),
        _ => value.clone(),
    }
}

pub fn validate(config: &Value, rationale: &RationaleFile) -> (Vec<Finding>, Vec<Coverage>) {
    let mut findings = Vec::new();
    if rationale.version != 1 {
        findings.push(error(
            "unsupported_version",
            "",
            format!(
                "sidecar version {} is unsupported; expected 1",
                rationale.version
            ),
        ));
    }

    let mut seen = BTreeSet::new();
    let decisions: BTreeMap<_, _> = rationale
        .decisions
        .iter()
        .filter_map(|decision| {
            if !seen.insert(decision.path.as_str()) {
                findings.push(error(
                    "duplicate_target",
                    &decision.path,
                    "more than one decision targets this path".into(),
                ));
                None
            } else {
                Some((decision.path.as_str(), decision))
            }
        })
        .collect();

    for decision in &rationale.decisions {
        let Some(value) = pointer(config, &decision.path) else {
            findings.push(error(
                "orphaned_rationale",
                &decision.path,
                "target no longer exists in the configuration".into(),
            ));
            continue;
        };
        if !has_rationale(decision) {
            findings.push(error(
                "missing_rationale",
                &decision.path,
                "replace TODO with a concrete rationale".into(),
            ));
        }
        if decision.value_hash != fingerprint(value) {
            findings.push(error(
                "stale_rationale",
                &decision.path,
                "the target value changed; review the rationale and run `crg stamp`".into(),
            ));
        }
        if let Some(date) = &decision.review_by {
            match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                Ok(date) if date < Utc::now().date_naive() => findings.push(error(
                    "review_overdue",
                    &decision.path,
                    format!("review date {date} has passed"),
                )),
                Ok(_) => {}
                Err(_) => findings.push(error(
                    "invalid_review_date",
                    &decision.path,
                    "reviewBy must use YYYY-MM-DD".into(),
                )),
            }
        }
        if decision.policy.as_deref() == Some("") {
            findings.push(warning(
                "empty_policy",
                &decision.path,
                "remove the empty policy field or add a policy reference".into(),
            ));
        }
    }

    let leaves = leaf_paths(config);
    let mut coverage = Vec::new();
    for rule in &rationale.rules {
        if rule.pattern.is_empty() || !rule.pattern.starts_with('/') {
            findings.push(error(
                "invalid_rule",
                &rule.pattern,
                "rule patterns must be JSON Pointer-like paths beginning with /".into(),
            ));
            continue;
        }
        if !(0.0..=1.0).contains(&rule.minimum_coverage) {
            findings.push(error(
                "invalid_rule",
                &rule.pattern,
                "minimumCoverage must be between 0 and 1".into(),
            ));
            continue;
        }
        let targets: Vec<_> = leaves
            .iter()
            .filter(|path| pattern_matches(&rule.pattern, path))
            .collect();
        let covered = targets
            .iter()
            .filter(|path| {
                decisions
                    .get(path.as_str())
                    .is_some_and(|d| has_rationale(d))
            })
            .count();
        let ratio = if targets.is_empty() {
            1.0
        } else {
            covered as f64 / targets.len() as f64
        };
        if ratio + f64::EPSILON < rule.minimum_coverage {
            findings.push(error(
                "coverage_below_minimum",
                &rule.pattern,
                format!(
                    "{covered}/{} targets have rationale ({:.0}%); requires {:.0}%",
                    targets.len(),
                    ratio * 100.0,
                    rule.minimum_coverage * 100.0
                ),
            ));
        }
        coverage.push(Coverage {
            pattern: rule.pattern.clone(),
            covered,
            total: targets.len(),
            ratio,
            minimum: rule.minimum_coverage,
        });
    }
    (findings, coverage)
}

pub fn has_rationale(decision: &Decision) -> bool {
    let text = decision.rationale.trim();
    text.len() >= 12 && !text.eq_ignore_ascii_case("todo")
}

pub fn error(code: &'static str, path: &str, message: String) -> Finding {
    Finding {
        severity: Severity::Error,
        code,
        path: path.to_owned(),
        message,
    }
}

pub fn warning(code: &'static str, path: &str, message: String) -> Finding {
    Finding {
        severity: Severity::Warning,
        code,
        path: path.to_owned(),
        message,
    }
}
