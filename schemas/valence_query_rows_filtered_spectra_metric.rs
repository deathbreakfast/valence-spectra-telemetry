use spectra_macros::spectra_metric;

spectra_metric! {
    ValenceQueryRowsFiltered {
        store: "valence",
        name: "valence_query_rows_filtered",
        version: "0.1.0",
        description: "Valence query rows removed by privacy or pending-deletion post-filters. Labels: primary_table, filter_kind.",
        level: Debug,
    }
}
