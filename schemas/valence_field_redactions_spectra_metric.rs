use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceFieldRedactions {
        store: "valence",
        name: "valence_field_redactions",
        version: "0.1.0",
        description: "Valence field-level privacy redactions in filter_entity_fields. Labels: table, field.",
        level: Debug,
    }
}
