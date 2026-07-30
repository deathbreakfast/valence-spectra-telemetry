use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceOwnershipTransfers {
        store: "valence",
        name: "valence_ownership_transfers",
        version: "0.1.0",
        description: "Valence ownership transfers. Labels: table.",
    }
}
