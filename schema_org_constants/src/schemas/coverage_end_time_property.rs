/// <https://schema.org/coverageEndTime>
pub const COVERAGE_END_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/coverageEndTime";
/// <https://schema.org/coverageEndTime>
pub const COVERAGE_END_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/coverageEndTime";
/// <https://schema.org/coverageEndTime>
pub const COVERAGE_END_TIME_PROPERTY_LABEL: &str = "coverageEndTime";
pub struct CoverageEndTimePropertyIri;
impl PartialEq<&str> for CoverageEndTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COVERAGE_END_TIME_PROPERTY_IRI_HTTP
			|| *other == COVERAGE_END_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CoverageEndTimePropertyIri> for &str {
	fn eq(&self, other: &CoverageEndTimePropertyIri) -> bool {
		*self == COVERAGE_END_TIME_PROPERTY_IRI_HTTP
			|| *self == COVERAGE_END_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct CoverageEndTimePropertyIriOrLabel;
impl PartialEq<&str> for CoverageEndTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoverageEndTimePropertyIri || *other == COVERAGE_END_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<CoverageEndTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CoverageEndTimePropertyIriOrLabel) -> bool {
		*self == CoverageEndTimePropertyIri || *self == COVERAGE_END_TIME_PROPERTY_LABEL
	}
}
