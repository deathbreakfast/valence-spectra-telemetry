use std::sync::{Arc, OnceLock};

use valence_telemetry::{install_telemetry_sink, ConsoleSink, NoOpSink, TelemetrySink};

use crate::metrics;

static INSTALLED_SINK: OnceLock<Arc<dyn TelemetrySink>> = OnceLock::new();

/// Spectra-backed Valence telemetry sink.
///
/// # Examples
///
/// ```rust,no_run
/// use valence_spectra_telemetry::SpectraTelemetrySink;
/// use valence_telemetry::TelemetrySink;
///
/// let sink = SpectraTelemetrySink::new();
/// sink.record_counter(
///     "valence_db_reads",
///     &[("table", "users"), ("database_type", "sqlite")],
///     1,
/// );
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct SpectraTelemetrySink;

impl SpectraTelemetrySink {
    /// Build a Spectra-backed sink.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use valence_spectra_telemetry::SpectraTelemetrySink;
    ///
    /// let _sink = SpectraTelemetrySink::new();
    /// ```
    pub const fn new() -> Self {
        Self
    }
}

impl TelemetrySink for SpectraTelemetrySink {
    fn record_counter(&self, name: &str, labels: &[(&str, &str)], delta: u64) {
        metrics::record_counter(name, labels, delta);
    }

    fn record_gauge(&self, name: &str, labels: &[(&str, &str)], value: f64) {
        metrics::record_gauge(name, labels, value);
    }

    fn log_event(&self, schema: &str, fields: &[(&str, &str)]) {
        metrics::log_event(schema, fields);
    }

    fn log_event_value(&self, schema: &str, payload: &serde_json::Value) {
        metrics::log_event_value(schema, payload);
    }
}

/// Resolved Valence telemetry backend from `VALENCE_TELEMETRY`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SinkKind {
    NoOp,
    Console,
    Spectra,
}

fn sink_kind_from_env_value(raw: Option<&str>) -> SinkKind {
    match raw
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("off" | "0" | "false" | "none") => SinkKind::NoOp,
        Some("console") => SinkKind::Console,
        _ => SinkKind::Spectra,
    }
}

fn sink_from_env() -> Arc<dyn TelemetrySink> {
    match sink_kind_from_env_value(std::env::var("VALENCE_TELEMETRY").ok().as_deref()) {
        SinkKind::NoOp => Arc::new(NoOpSink),
        SinkKind::Console => Arc::new(ConsoleSink),
        SinkKind::Spectra => Arc::new(SpectraTelemetrySink::new()),
    }
}

/// Resolve Valence telemetry sink from `VALENCE_TELEMETRY` and install process-global dispatch.
///
/// - `off` | `0` | `false` | `none` → [`NoOpSink`]
/// - `console` → [`ConsoleSink`]
/// - default (including `spectra`) → [`SpectraTelemetrySink`] when Spectra sink is configured
///
/// # Examples
///
/// ```rust,no_run
/// use valence_spectra_telemetry::install_from_env;
///
/// let _sink = install_from_env();
/// ```
pub fn install_from_env() -> Arc<dyn TelemetrySink> {
    let sink = Arc::clone(INSTALLED_SINK.get_or_init(|| {
        let sink = sink_from_env();
        install_telemetry_sink(Arc::clone(&sink));
        sink
    }));
    install_telemetry_sink(Arc::clone(&sink));
    sink
}

#[cfg(test)]
mod tests {
    #![allow(missing_docs)]

    use super::{sink_kind_from_env_value, SinkKind};

    #[test]
    fn sink_kind_off_aliases_and_console_happy() {
        for off in ["off", "0", "false", "none", " OFF ", "False"] {
            assert_eq!(
                sink_kind_from_env_value(Some(off)),
                SinkKind::NoOp,
                "expected NoOp for {off:?}"
            );
        }
        assert_eq!(sink_kind_from_env_value(Some("console")), SinkKind::Console);
        assert_eq!(
            sink_kind_from_env_value(Some(" Console ")),
            SinkKind::Console
        );
        assert_eq!(sink_kind_from_env_value(Some("spectra")), SinkKind::Spectra);
        assert_eq!(sink_kind_from_env_value(None), SinkKind::Spectra);
    }

    #[test]
    fn sink_kind_unknown_or_empty_defaults_to_spectra_sad() {
        assert_eq!(
            sink_kind_from_env_value(Some("not-a-backend")),
            SinkKind::Spectra
        );
        assert_eq!(sink_kind_from_env_value(Some("")), SinkKind::Spectra);
        assert_eq!(sink_kind_from_env_value(Some("   ")), SinkKind::Spectra);
    }
}
