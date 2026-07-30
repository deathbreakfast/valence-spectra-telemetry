use spectra_macros::spectra_schema;

spectra_schema! {
    ValenceSlowOp {
        store: "valence",
        table: "valence_slow_op",
        version: "0.1.0",
        description: "DatabaseBackend call exceeding VALENCE_SLOW_OP_MS threshold (operator forensics).",
        level: Warn,
        fields: [
            operation: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
            table: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
            op: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
            database_type: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
            wall_ms: {
                r#type: Float,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
            record_id: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
        ],
    }
}
