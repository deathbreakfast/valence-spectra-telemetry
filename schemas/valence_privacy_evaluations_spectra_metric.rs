use spectra_macros::spectra_metric;

spectra_metric! {
    ValencePrivacyEvaluations {
        store: "valence",
        name: "valence_privacy_evaluations",
        version: "0.1.0",
        description: "Per-rule PrivacyEvaluator evaluations. Labels: policy, actor_kind, operation, table, matched.",
        level: Debug,
    }
}
