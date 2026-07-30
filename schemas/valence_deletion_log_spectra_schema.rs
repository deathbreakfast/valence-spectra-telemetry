use spectra_macros::spectra_schema;

spectra_schema! {
    ValenceDeletionLog {
        store: "valence",
        table: "valence_deletion_log",
        version: "0.1.0",
        description: "Valence deletion framework DAG/run telemetry (no child row payloads).",
        fields: [
            outcome: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            root_table: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            root_record_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            node_count: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            max_depth: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            restrict_count: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            trait_expansion_count: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
        ],
    }
}
