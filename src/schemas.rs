//! Spectra metric schema registration for Valence Spectra metrics and events.
//!
//! Event schemas under `schemas/*_spectra_schema.rs` that declare a user field named
//! `table` are **not** compiled here: crates.io `spectra-macros` still emits the schema
//! identity slot as a Rust field also named `table`, which collides. Those event helpers
//! stay in [`crate::codegen`] / `src/generated.rs`. See `schemas/README.md`.

mod valence_db_reads_metric {
    include!("../schemas/valence_db_reads_spectra_metric.rs");
}

mod valence_db_writes_metric {
    include!("../schemas/valence_db_writes_spectra_metric.rs");
}

mod valence_db_errors_metric {
    include!("../schemas/valence_db_errors_spectra_metric.rs");
}

mod valence_edge_reads_metric {
    include!("../schemas/valence_edge_reads_spectra_metric.rs");
}

mod valence_edge_writes_metric {
    include!("../schemas/valence_edge_writes_spectra_metric.rs");
}

mod valence_privacy_denials_metric {
    include!("../schemas/valence_privacy_denials_spectra_metric.rs");
}

mod valence_privacy_evaluations_metric {
    include!("../schemas/valence_privacy_evaluations_spectra_metric.rs");
}

mod valence_query_compile_errors_metric {
    include!("../schemas/valence_query_compile_errors_spectra_metric.rs");
}

mod valence_query_executions_metric {
    include!("../schemas/valence_query_executions_spectra_metric.rs");
}

mod valence_query_results_capped_metric {
    include!("../schemas/valence_query_results_capped_spectra_metric.rs");
}

mod valence_query_rows_filtered_metric {
    include!("../schemas/valence_query_rows_filtered_spectra_metric.rs");
}

mod valence_validation_failures_metric {
    include!("../schemas/valence_validation_failures_spectra_metric.rs");
}

mod valence_unique_violations_metric {
    include!("../schemas/valence_unique_violations_spectra_metric.rs");
}

mod valence_field_redactions_metric {
    include!("../schemas/valence_field_redactions_spectra_metric.rs");
}

mod valence_router_resolve_errors_metric {
    include!("../schemas/valence_router_resolve_errors_spectra_metric.rs");
}

mod valence_deserialize_errors_metric {
    include!("../schemas/valence_deserialize_errors_spectra_metric.rs");
}

mod valence_deletion_run_queued_metric {
    include!("../schemas/valence_deletion_run_queued_spectra_metric.rs");
}

mod valence_deletion_restrict_blocked_metric {
    include!("../schemas/valence_deletion_restrict_blocked_spectra_metric.rs");
}

mod valence_ownership_transfers_metric {
    include!("../schemas/valence_ownership_transfers_spectra_metric.rs");
}

mod valence_query_wall_ms_metric {
    include!("../schemas/valence_query_wall_ms_spectra_metric.rs");
}

mod valence_mutation_wall_ms_metric {
    include!("../schemas/valence_mutation_wall_ms_spectra_metric.rs");
}

mod valence_db_wall_ms_metric {
    include!("../schemas/valence_db_wall_ms_spectra_metric.rs");
}

mod valence_deletion_dag_nodes_metric {
    include!("../schemas/valence_deletion_dag_nodes_spectra_metric.rs");
}

mod valence_deletion_dag_max_depth_metric {
    include!("../schemas/valence_deletion_dag_max_depth_spectra_metric.rs");
}

mod valence_side_effects_metric {
    include!("../schemas/valence_side_effects_spectra_metric.rs");
}
