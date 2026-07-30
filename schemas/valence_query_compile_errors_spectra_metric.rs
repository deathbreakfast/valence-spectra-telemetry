use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceQueryCompileErrors {
        store: "valence",
        name: "valence_query_compile_errors",
        version: "0.1.0",
        description: "Valence query SQL compilation failures. Labels: primary_table, query_target.",
        level: Error,
    }
}
