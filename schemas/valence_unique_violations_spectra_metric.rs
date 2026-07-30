use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceUniqueViolations {
        store: "valence",
        name: "valence_unique_violations",
        version: "0.1.0",
        description: "Valence unique constraint violations on create/update. Labels: table, field.",
        level: Error,
    }
}
