use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceMutationWallMs {
        store: "valence",
        name: "valence_mutation_wall_ms",
        version: "0.1.0",
        description: "Valence mutation wall-clock latency in milliseconds. Labels: operation.",
        level: Trace,
        default_sample_rate: 0.1,
    }
}
