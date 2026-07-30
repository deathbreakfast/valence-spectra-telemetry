use spectra_macros::spectra_schema;

spectra_schema! {
    ValenceQueryLog {
        store: "valence",
        table: "valence_query_log",
        version: "0.1.0",
        description: "Per-query execution profile row (no row payloads).",
        level: Debug,
        fields: [
            outcome: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            query_target: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            trait_name: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            primary_table: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            implementor_count: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            hop_count: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            has_limit: {
                r#type: bool,
                classification: { pii: false, safe_for_console: true },
            },
            limit: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            offset: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            rows_db: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            rows_after_privacy: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            rows_after_pending_deletion: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            wall_ms: {
                r#type: i64,
                classification: { pii: false, safe_for_console: true },
            },
            compile_error: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            caller: {
                r#type: String,
                classification: { pii: true, safe_for_console: false },
            },
        ],
    }
}
