use spectra_macros::spectra_schema;

spectra_schema! {
    ValencePrivacyEval {
        store: "valence",
        table: "valence_privacy_eval",
        version: "0.1.0",
        description: "Per-rule privacy policy evaluation (audit). No record payloads.",
        level: Debug,
        fields: [
            policy: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            actor_kind: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            actor_id: {
                r#type: String,
                classification: { pii: true, safe_for_console: false },
            },
            operation: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            table: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            rule_phase: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
            matched: {
                r#type: bool,
                classification: { pii: false, safe_for_console: false },
            },
            check_outcome: {
                r#type: String,
                classification: { pii: false, safe_for_console: false },
            },
        ],
    }
}
