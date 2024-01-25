/// <https://schema.org/firstPerformance>
pub const FIRST_PERFORMANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/firstPerformance";
/// <https://schema.org/firstPerformance>
pub const FIRST_PERFORMANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/firstPerformance";
/// <https://schema.org/firstPerformance>
pub const FIRST_PERFORMANCE_PROPERTY_LABEL: &str = "firstPerformance";
pub struct FirstPerformancePropertyIri;
impl PartialEq<&str> for FirstPerformancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FIRST_PERFORMANCE_PROPERTY_IRI_HTTP
			|| *other == FIRST_PERFORMANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<FirstPerformancePropertyIri> for &str {
	fn eq(&self, other: &FirstPerformancePropertyIri) -> bool {
		*self == FIRST_PERFORMANCE_PROPERTY_IRI_HTTP
			|| *self == FIRST_PERFORMANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct FirstPerformancePropertyIriOrLabel;
impl PartialEq<&str> for FirstPerformancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FirstPerformancePropertyIri || *other == FIRST_PERFORMANCE_PROPERTY_LABEL
	}
}
impl PartialEq<FirstPerformancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &FirstPerformancePropertyIriOrLabel) -> bool {
		*self == FirstPerformancePropertyIri || *self == FIRST_PERFORMANCE_PROPERTY_LABEL
	}
}
