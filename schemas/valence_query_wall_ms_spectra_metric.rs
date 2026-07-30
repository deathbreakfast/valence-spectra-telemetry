use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceQueryWallMs {
        store: "valence",
        name: "valence_query_wall_ms",
        version: "0.1.0",
        description: "Valence query wall-clock latency in milliseconds. Labels: primary_table, query_target.",
        level: Trace,
        default_sample_rate: 0.1,
    }
}
