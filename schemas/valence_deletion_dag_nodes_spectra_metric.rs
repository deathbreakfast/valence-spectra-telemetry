use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDeletionDagNodes {
        store: "valence",
        name: "valence_deletion_dag_nodes",
        version: "0.1.0",
        description: "Valence deletion DAG node count at compute time. Labels: root_table.",
    }
}
