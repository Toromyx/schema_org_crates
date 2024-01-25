/// <https://schema.org/coverageStartTime>
pub const COVERAGE_START_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/coverageStartTime";
/// <https://schema.org/coverageStartTime>
pub const COVERAGE_START_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/coverageStartTime";
/// <https://schema.org/coverageStartTime>
pub const COVERAGE_START_TIME_PROPERTY_LABEL: &str = "coverageStartTime";
pub struct CoverageStartTimePropertyIri;
impl PartialEq<&str> for CoverageStartTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COVERAGE_START_TIME_PROPERTY_IRI_HTTP
			|| *other == COVERAGE_START_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CoverageStartTimePropertyIri> for &str {
	fn eq(&self, other: &CoverageStartTimePropertyIri) -> bool {
		*self == COVERAGE_START_TIME_PROPERTY_IRI_HTTP
			|| *self == COVERAGE_START_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CoverageStartTimePropertyIriOrLabel;
impl PartialEq<&str> for CoverageStartTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoverageStartTimePropertyIri || *other == COVERAGE_START_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CoverageStartTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CoverageStartTimePropertyIriOrLabel) -> bool {
		*self == CoverageStartTimePropertyIri || *self == COVERAGE_START_TIME_PROPERTY_LABEL
	}
}
