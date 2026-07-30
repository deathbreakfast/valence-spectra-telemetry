use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceQueryResultsCapped {
        store: "valence",
        name: "valence_query_results_capped",
        version: "0.1.0",
        description: "Valence queries where result count hit the configured limit. Labels: primary_table, query_target.",
    }
}
