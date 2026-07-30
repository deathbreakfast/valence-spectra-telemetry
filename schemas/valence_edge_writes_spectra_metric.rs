use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceEdgeWrites {
        store: "valence",
        name: "valence_edge_writes",
        version: "0.1.0",
        description: "Valence graph edge writes (relate, unrelate). Labels: edge_table, database_type, op.",
    }
}
