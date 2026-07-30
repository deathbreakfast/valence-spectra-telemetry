# Valence Spectra DSL sources

Human-edited Spectra DSL for Valence metrics and events.

## Metrics (`*_spectra_metric.rs`)

Compiled into the crate via `src/schemas.rs` so `spectra_metric!` inventory registration
runs when this crate is linked.

## Events (`*_spectra_schema.rs`)

Kept as the editable source of truth for event shapes. Several event schemas declare a
**user field** named `table` (Valence’s wire key). Current crates.io `spectra-macros`
also emits the schema identity slot as a Rust field named `table`, which collides at
compile time.

Until Spectra macros emit that identity slot as `schema_table` (or otherwise allow a
user field named `table`), event inventory is **not** compiled here. Typed event
helpers / topic DTOs live in `src/generated.rs` instead.

When regenerating helpers after a macros fix, prefer restoring event `include!`s in
`src/schemas.rs` and dropping or regenerating `src/generated.rs` from the same DSL.
