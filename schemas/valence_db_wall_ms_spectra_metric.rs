use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDbWallMs {
        store: "valence",
        name: "valence_db_wall_ms",
        version: "0.1.0",
        description: "Per-table DatabaseBackend I/O wall-clock latency in milliseconds. Labels: table, op, database_type.",
        level: Trace,
        default_sample_rate: 1.0,
    }
}
