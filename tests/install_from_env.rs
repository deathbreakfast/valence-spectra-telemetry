//! Process-isolated install coverage (`OnceLock` caches the first resolution).
#![allow(missing_docs)]

use valence_spectra_telemetry::install_from_env;

#[test]
fn install_from_env_off_returns_usable_sink_happy() {
    std::env::set_var("VALENCE_TELEMETRY", "off");
    let sink = install_from_env();
    sink.record_counter("valence_db_reads", &[("table", "t"), ("op", "get")], 1);
    sink.record_gauge("valence_query_wall_ms", &[("table", "t")], 1.5);
    sink.log_event("valence_slow_op", &[("op", "query"), ("table", "t")]);
    sink.log_event_value("valence_error_log", &serde_json::json!({"message": "x"}));
}
