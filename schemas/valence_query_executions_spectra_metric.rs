use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceQueryExecutions {
        store: "valence",
        name: "valence_query_executions",
        version: "0.1.0",
        description: "Valence query executions (successful compile+execute). Labels: primary_table, query_target, hop_count.",
        level: Debug,
    }
}
