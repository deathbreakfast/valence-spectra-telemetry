use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDbWrites {
        store: "valence",
        name: "valence_db_writes",
        version: "0.1.0",
        description: "Valence database write operations (create, update, upsert, merge, delete). Labels: table, database_type, op.",
    }
}
