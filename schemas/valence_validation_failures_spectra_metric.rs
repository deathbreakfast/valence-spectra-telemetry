use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceValidationFailures {
        store: "valence",
        name: "valence_validation_failures",
        version: "0.1.0",
        description: "Valence field validation failures before backend I/O. Labels: table, field, validator.",
        level: Error,
    }
}
