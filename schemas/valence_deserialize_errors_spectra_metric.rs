use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceDeserializeErrors {
        store: "valence",
        name: "valence_deserialize_errors",
        version: "0.1.0",
        description: "Valence query row deserialization failures (schema drift). Labels: table.",
        level: Error,
    }
}
