use spectra_macros::spectra_schema;

spectra_schema! {
    ValenceOwnershipLog {
        store: "valence",
        table: "valence_ownership_log",
        version: "0.1.0",
        description: "Valence ownership transfer audit (no record payloads).",
        fields: [
            table: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            record_id: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            from_owner_id: {
                r#type: String,
                classification: { pii: true, safe_for_console: false },
            },
            from_owner_type: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            to_owner_id: {
                r#type: String,
                classification: { pii: true, safe_for_console: false },
            },
            to_owner_type: {
                r#type: String,
                classification: { pii: false, safe_for_console: true },
            },
            actor: {
                r#type: String,
                classification: { pii: true, safe_for_console: false },
            },
        ],
    }
}
