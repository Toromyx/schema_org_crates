/// <https://schema.org/temporalCoverage>
pub const TEMPORAL_COVERAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/temporalCoverage";
/// <https://schema.org/temporalCoverage>
pub const TEMPORAL_COVERAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/temporalCoverage";
/// <https://schema.org/temporalCoverage>
pub const TEMPORAL_COVERAGE_PROPERTY_LABEL: &str = "temporalCoverage";
pub struct TemporalCoveragePropertyIri;
impl PartialEq<&str> for TemporalCoveragePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TEMPORAL_COVERAGE_PROPERTY_IRI_HTTP
			|| *other == TEMPORAL_COVERAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TemporalCoveragePropertyIri> for &str {
	fn eq(&self, other: &TemporalCoveragePropertyIri) -> bool {
		*self == TEMPORAL_COVERAGE_PROPERTY_IRI_HTTP
			|| *self == TEMPORAL_COVERAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct TemporalCoveragePropertyIriOrLabel;
impl PartialEq<&str> for TemporalCoveragePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TemporalCoveragePropertyIri || *other == TEMPORAL_COVERAGE_PROPERTY_LABEL
	}
}
impl PartialEq<TemporalCoveragePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TemporalCoveragePropertyIriOrLabel) -> bool {
		*self == TemporalCoveragePropertyIri || *self == TEMPORAL_COVERAGE_PROPERTY_LABEL
	}
}
