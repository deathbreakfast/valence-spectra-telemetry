use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceSideEffects {
        store: "valence",
        name: "valence_side_effects",
        version: "0.1.0",
        description: "Valence side-effect dispatches after successful mutations. Labels: table, kind.",
    }
}
