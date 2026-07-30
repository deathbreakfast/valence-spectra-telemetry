use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDeletionRestrictBlocked {
        store: "valence",
        name: "valence_deletion_restrict_blocked",
        version: "0.1.0",
        description: "Valence deletion blocked by restrict violations. Labels: root_table, connection_name.",
        level: Warn,
    }
}
