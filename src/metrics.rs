//! Crate-internal remapping from [`valence_telemetry::TelemetrySink`] calls onto `spectra-core`.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde_json::Value;
use spectra_core::{try_log_event, try_record_counter, try_record_gauge};

use crate::sanitize::sanitize_error_message;

const MAX_EVENT_TEXT_LEN: usize = 512;
const ELLIPSIS: &str = "…";

const SANITIZED_ERROR_FIELDS: &[&str] = &["message", "compile_error"];

const MASKED_EVENT_FIELDS: &[(&str, &str)] = &[("valence_query_log", "caller")];

const BOUNDED_EVENT_SCHEMAS: &[&str] = &[
    "valence_error_log",
    "valence_query_log",
    "valence_slow_op",
    "valence_privacy_eval",
    "valence_ownership_log",
    "valence_deletion_log",
];

/// Convert a Valence `u64` counter delta to Spectra's `i64`, saturating on overflow.
const fn counter_delta(delta: u64) -> i64 {
    if delta > i64::MAX as u64 {
        i64::MAX
    } else {
        delta.cast_signed()
    }
}

fn hash_label_value(value: &str) -> String {
    let mut hasher = DefaultHasher::new();
    value.trim().hash(&mut hasher);
    format!("h{:016x}", hasher.finish())
}

fn truncate_event_text(text: &str) -> String {
    if text.len() <= MAX_EVENT_TEXT_LEN {
        return text.to_owned();
    }

    let budget = MAX_EVENT_TEXT_LEN.saturating_sub(ELLIPSIS.len());
    let boundary = text.floor_char_boundary(budget);
    format!("{}{ELLIPSIS}", &text[..boundary])
}

fn should_mask_event_field(schema: &str, key: &str) -> bool {
    MASKED_EVENT_FIELDS
        .iter()
        .any(|(table, field)| *table == schema && *field == key)
}

fn bound_event_string(schema: &str, key: &str, value: &str) -> String {
    if should_mask_event_field(schema, key) {
        return hash_label_value(value);
    }
    if SANITIZED_ERROR_FIELDS.contains(&key) {
        return sanitize_error_message(value);
    }
    if BOUNDED_EVENT_SCHEMAS.contains(&schema) {
        return truncate_event_text(value);
    }
    value.to_owned()
}

fn metric_label_keys(name: &str) -> Option<&'static [&'static str]> {
    match name {
        "valence_db_reads" | "valence_db_writes" => Some(&["table", "database_type", "op"]),
        "valence_db_errors" => Some(&["operation", "database_type"]),
        "valence_db_wall_ms" => Some(&["table", "op", "database_type"]),
        "valence_edge_reads" => Some(&["edge_table", "database_type"]),
        "valence_edge_writes" => Some(&["edge_table", "database_type", "op"]),
        "valence_privacy_denials" => Some(&["table", "policy"]),
        "valence_privacy_evaluations" => {
            Some(&["policy", "actor_kind", "operation", "table", "matched"])
        }
        "valence_query_compile_errors"
        | "valence_query_results_capped"
        | "valence_query_rows_filtered"
        | "valence_query_wall_ms" => Some(&["primary_table", "query_target"]),
        "valence_query_executions" => Some(&["primary_table", "query_target", "hop_count"]),
        "valence_mutation_wall_ms" => Some(&["operation"]),
        "valence_field_redactions" | "valence_unique_violations" => Some(&["table", "field"]),
        "valence_validation_failures" => Some(&["table", "field", "validator"]),
        "valence_deserialize_errors"
        | "valence_router_resolve_errors"
        | "valence_ownership_transfers" => Some(&["table"]),
        "valence_side_effects" => Some(&["table", "kind"]),
        "valence_deletion_run_queued"
        | "valence_deletion_dag_nodes"
        | "valence_deletion_dag_max_depth" => Some(&["root_table"]),
        "valence_deletion_restrict_blocked" => Some(&["root_table", "connection_name"]),
        _ => None,
    }
}

fn filter_labels(name: &str, labels: &[(&str, &str)]) -> Option<Vec<(String, String)>> {
    let allowed = metric_label_keys(name)?;
    let mut out = Vec::new();
    for (key, value) in labels {
        if allowed.contains(key) {
            out.push(((*key).to_owned(), (*value).to_owned()));
        }
    }
    Some(out)
}

fn bound_event_payload(schema: &str, payload: &Value) -> Value {
    let Some(object) = payload.as_object() else {
        return payload.clone();
    };

    let mut out = object.clone();
    for (key, value) in &mut out {
        if let Value::String(text) = value {
            *value = Value::String(bound_event_string(schema, key, text));
        }
    }
    Value::Object(out)
}

/// Handle a [`valence_telemetry::TelemetrySink::record_counter`] call.
pub fn record_counter(name: &str, labels: &[(&str, &str)], delta: u64) {
    let Some(filtered) = filter_labels(name, labels) else {
        return;
    };
    let refs: Vec<(&str, &str)> = filtered
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    try_record_counter(name, &refs, counter_delta(delta));
}

/// Handle a [`valence_telemetry::TelemetrySink::record_gauge`] call.
pub fn record_gauge(name: &str, labels: &[(&str, &str)], value: f64) {
    let Some(filtered) = filter_labels(name, labels) else {
        return;
    };
    let refs: Vec<(&str, &str)> = filtered
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    try_record_gauge(name, &refs, value);
}

/// Handle a [`valence_telemetry::TelemetrySink::log_event`] call by converting the
/// borrowed string fields into a JSON object.
pub fn log_event(schema: &str, fields: &[(&str, &str)]) {
    let object = fields
        .iter()
        .fold(serde_json::Map::new(), |mut acc, &(key, value)| {
            acc.insert(
                key.to_owned(),
                Value::String(bound_event_string(schema, key, value)),
            );
            acc
        });
    try_log_event(schema, &Value::Object(object));
}

/// Handle a [`valence_telemetry::TelemetrySink::log_event_value`] call, forwarding an
/// already-structured JSON payload after bounding untrusted text fields.
pub fn log_event_value(schema: &str, payload: &Value) {
    try_log_event(schema, &bound_event_payload(schema, payload));
}

#[cfg(test)]
mod tests {
    #![allow(missing_docs)]

    use super::{
        bound_event_payload, bound_event_string, counter_delta, filter_labels, metric_label_keys,
        truncate_event_text, MAX_EVENT_TEXT_LEN,
    };
    use serde_json::json;

    #[test]
    fn counter_delta_in_range_values_happy() {
        assert_eq!(counter_delta(0), 0);
        assert_eq!(counter_delta(1), 1);
        assert_eq!(counter_delta(42), 42);
        assert_eq!(counter_delta(i64::MAX as u64), i64::MAX);
    }

    #[test]
    fn counter_delta_overflow_saturates_to_i64_max_sad() {
        assert_eq!(counter_delta(u64::MAX), i64::MAX);
        assert_eq!(counter_delta((i64::MAX as u64) + 1), i64::MAX);
    }

    #[test]
    fn truncate_event_text_preserves_utf8_and_limit() {
        assert_eq!(truncate_event_text("ok"), "ok");

        let text = "é".repeat(300);
        let truncated = truncate_event_text(&text);
        assert!(truncated.len() <= MAX_EVENT_TEXT_LEN);
        assert!(truncated.ends_with('…'));
        assert!(truncated.is_char_boundary(truncated.len()));
    }

    #[test]
    fn bound_event_string_truncates_slow_op_fields_sad() {
        let long = "x".repeat(800);
        let bounded = bound_event_string("valence_slow_op", "table", &long);
        assert!(bounded.len() <= MAX_EVENT_TEXT_LEN);
        assert!(bounded.ends_with('…'));
    }

    #[test]
    fn bound_event_string_sanitizes_error_message_sad() {
        let msg = bound_event_string(
            "valence_error_log",
            "message",
            "validation failed password=hunter2",
        );
        assert!(msg.contains("[redacted]"));
        assert!(!msg.contains("hunter2"));
    }

    #[test]
    fn bound_event_string_masks_query_log_caller_sad() {
        let caller = bound_event_string("valence_query_log", "caller", "principal-42");
        assert_ne!(caller, "principal-42");
        assert!(caller.starts_with('h'));
        assert_eq!(caller.len(), 17);
    }

    #[test]
    fn bound_event_payload_truncates_all_string_fields_sad() {
        let long = "z".repeat(800);
        let payload = json!({
            "policy": long,
            "actor_kind": long,
            "operation": "read",
        });
        let bounded = bound_event_payload("valence_privacy_eval", &payload);
        for key in ["policy", "actor_kind"] {
            let text = bounded[key].as_str().unwrap_or("");
            assert!(text.len() <= MAX_EVENT_TEXT_LEN);
            assert!(text.ends_with('…'));
        }
        assert_eq!(bounded["operation"], "read");
    }

    #[test]
    fn filter_labels_drops_unknown_keys_sad() {
        let labels = [
            ("table", "users"),
            ("op", "get"),
            ("database_type", "mem"),
            ("extra", "drop"),
        ];
        let filtered = filter_labels("valence_db_reads", &labels).expect("known metric");
        assert_eq!(filtered.len(), 3);
        assert!(filtered.iter().all(|(k, _)| k != "extra"));
    }

    #[test]
    fn unknown_metric_has_no_label_keys_sad() {
        assert!(metric_label_keys("unknown_valence_metric").is_none());
        assert!(filter_labels("unknown_valence_metric", &[]).is_none());
    }
}
