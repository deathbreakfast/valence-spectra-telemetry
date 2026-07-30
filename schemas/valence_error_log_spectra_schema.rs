use spectra_macros::spectra_schema;

spectra_schema! {
    ValenceErrorLog {
        store: "valence",
        table: "valence_error_log",
        version: "0.1.0",
        description: "Structured Valence error messages for operator dashboards (no row payloads).",
        level: Error,
        fields: [
            source: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
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
            database_type: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
            message: {
                r#type: String,
                classification: {
                    pii: false,
                    safe_for_console: false,
                },
            },
        ],
    }
}
