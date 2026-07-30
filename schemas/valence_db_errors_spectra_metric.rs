use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDbErrors {
        store: "valence",
        name: "valence_db_errors",
        version: "0.1.0",
        description: "Valence database operation failures (backend Err, retry exhaustion). Labels: operation, database_type.",
        level: Error,
    }
}
