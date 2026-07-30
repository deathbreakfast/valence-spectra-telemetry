use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceRouterResolveErrors {
        store: "valence",
        name: "valence_router_resolve_errors",
        version: "0.1.0",
        description: "Valence backend_for_table resolution failures. Labels: table.",
        level: Error,
    }
}
