use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDeletionRunQueued {
        store: "valence",
        name: "valence_deletion_run_queued",
        version: "0.1.0",
        description: "Valence cascade deletion runs queued. Labels: root_table.",
    }
}
