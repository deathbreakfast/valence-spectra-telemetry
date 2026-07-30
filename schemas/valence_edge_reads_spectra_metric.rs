use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceEdgeReads {
        store: "valence",
        name: "valence_edge_reads",
        version: "0.1.0",
        description: "Valence graph edge reads (get_edge_targets). Labels: edge_table, database_type.",
        level: Debug,
    }
}
