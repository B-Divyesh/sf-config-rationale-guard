use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Number, Value};
use std::{fmt, fs, path::Path};

pub fn load(path: &Path) -> Result<(Value, &'static str), String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let parsed = match extension.as_str() {
        "json" => {
            let mut deserializer = serde_json::Deserializer::from_str(&source);
            let value = StrictJson::deserialize(&mut deserializer)
                .map_err(|error| format!("invalid JSON in {}: {error}", path.display()))?
                .0;
            deserializer
                .end()
                .map_err(|error| format!("invalid JSON in {}: {error}", path.display()))?;
            (value, "json")
        }
        "yaml" | "yml" => {
            let yaml: serde_yaml::Value = serde_yaml::from_str(&source)
                .map_err(|error| format!("invalid YAML in {}: {error}", path.display()))?;
            let value = serde_json::to_value(yaml).map_err(|error| {
                format!(
                    "unsupported YAML mapping in {} (keys must be strings): {error}",
                    path.display()
                )
            })?;
            (value, "yaml")
        }
        "toml" => {
            let toml: toml::Value = toml::from_str(&source)
                .map_err(|error| format!("invalid TOML in {}: {error}", path.display()))?;
            let value = serde_json::to_value(toml)
                .map_err(|error| format!("could not normalize {}: {error}", path.display()))?;
            (value, "toml")
        }
        _ => {
            return Err(format!(
                "unsupported config format for {}; use .json, .yaml, .yml, or .toml",
                path.display()
            ));
        }
    };
    Ok(parsed)
}

struct StrictJson(Value);

impl<'de> Deserialize<'de> for StrictJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(JsonVisitor).map(StrictJson)
    }
}

struct JsonVisitor;

impl<'de> Visitor<'de> for JsonVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a strict JSON value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }
    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(value.into()))
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite number"))
    }
    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }
    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_seq<A>(self, mut sequence: A) -> Result<Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<StrictJson>()? {
            values.push(value.0);
        }
        Ok(Value::Array(values))
    }
    fn visit_map<A>(self, mut map: A) -> Result<Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some((key, value)) = map.next_entry::<String, StrictJson>()? {
            if values.insert(key.clone(), value.0).is_some() {
                return Err(de::Error::custom(format!("duplicate object key `{key}`")));
            }
        }
        Ok(Value::Object(values))
    }
}

pub fn pointer<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    if path.is_empty() {
        Some(root)
    } else {
        root.pointer(path)
    }
}

pub fn leaf_paths(root: &Value) -> Vec<String> {
    let mut output = Vec::new();
    collect_leaves(root, "", &mut output);
    output
}

fn collect_leaves(value: &Value, path: &str, output: &mut Vec<String>) {
    match value {
        Value::Object(values) if !values.is_empty() => {
            for (key, child) in values {
                collect_leaves(child, &format!("{path}/{}", escape(key)), output);
            }
        }
        Value::Array(values) if !values.is_empty() => {
            for (index, child) in values.iter().enumerate() {
                collect_leaves(child, &format!("{path}/{index}"), output);
            }
        }
        _ => output.push(path.to_owned()),
    }
}

fn escape(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

pub fn pattern_matches(pattern: &str, path: &str) -> bool {
    let patterns: Vec<_> = pattern.trim_start_matches('/').split('/').collect();
    let paths: Vec<_> = path.trim_start_matches('/').split('/').collect();
    matches_segments(&patterns, &paths)
}

fn matches_segments(pattern: &[&str], path: &[&str]) -> bool {
    match (pattern.first(), path.first()) {
        (None, None) => true,
        (Some(&"**"), _) => {
            matches_segments(&pattern[1..], path)
                || (!path.is_empty() && matches_segments(pattern, &path[1..]))
        }
        (Some(&"*"), Some(_)) => matches_segments(&pattern[1..], &path[1..]),
        (Some(left), Some(right)) if left == right => matches_segments(&pattern[1..], &path[1..]),
        _ => false,
    }
}
