use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDeletionDagMaxDepth {
        store: "valence",
        name: "valence_deletion_dag_max_depth",
        version: "0.1.0",
        description: "Valence deletion DAG max depth at compute time. Labels: root_table.",
    }
}
