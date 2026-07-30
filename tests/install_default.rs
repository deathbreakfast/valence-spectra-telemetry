//! Process-isolated default (`spectra`) install path.
#![allow(missing_docs)]

use valence_spectra_telemetry::install_from_env;

#[test]
fn install_from_env_default_uses_spectra_sink_happy() {
    std::env::remove_var("VALENCE_TELEMETRY");
    let sink = install_from_env();
    sink.record_counter(
        "valence_db_reads",
        &[("table", "t"), ("op", "get"), ("database_type", "mem")],
        1,
    );
    sink.log_event_value("valence_error_log", &serde_json::json!({"message": "ok"}));
}
