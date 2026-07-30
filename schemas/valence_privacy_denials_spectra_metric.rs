use spectra_macros::spectra_metric;

spectra_metric! {
    ValencePrivacyDenials {
        store: "valence",
        name: "valence_privacy_denials",
        version: "0.1.0",
        description: "Valence PrivacyEvaluator final access denials. Labels: table, policy.",
        level: Warn,
    }
}
