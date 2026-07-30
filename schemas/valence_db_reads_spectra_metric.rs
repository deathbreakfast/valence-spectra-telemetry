use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDbReads {
        store: "valence",
        name: "valence_db_reads",
        version: "0.1.0",
        description: "Valence database read operations (get_record, execute_compiled_query). Labels: table, database_type, op.",
        level: Debug,
    }
}
