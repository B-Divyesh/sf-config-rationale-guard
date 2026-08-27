use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RationaleFile {
    #[serde(default = "version")]
    pub version: u8,
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub decisions: Vec<Decision>,
}

fn version() -> u8 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rule {
    pub pattern: String,
    #[serde(default = "full_coverage")]
    pub minimum_coverage: f64,
}

fn full_coverage() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Decision {
    pub path: String,
    pub rationale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_by: Option<String>,
    pub value_hash: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub severity: Severity,
    pub code: &'static str,
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
    pub pattern: String,
    pub covered: usize,
    pub total: usize,
    pub ratio: f64,
    pub minimum: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckReport {
    pub valid: bool,
    pub config: String,
    pub rationale_file: String,
    pub format: String,
    pub schema_validated: bool,
    pub decisions: usize,
    pub coverage: Vec<Coverage>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffEntry {
    pub path: String,
    pub setting_change: &'static str,
    pub rationale_change: &'static str,
    pub rationale: Option<String>,
    pub policy: Option<String>,
    pub owner: Option<String>,
    pub review_by: Option<String>,
    pub status: &'static str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffReport {
    pub valid: bool,
    pub base: String,
    pub head: String,
    pub summary: DiffSummary,
    pub changes: Vec<DiffEntry>,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiffSummary {
    pub settings_changed: usize,
    pub rationales_changed: usize,
    pub attention_required: usize,
}
