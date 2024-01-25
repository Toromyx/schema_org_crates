/// <https://schema.org/performTime>
pub const PERFORM_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/performTime";
/// <https://schema.org/performTime>
pub const PERFORM_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/performTime";
/// <https://schema.org/performTime>
pub const PERFORM_TIME_PROPERTY_LABEL: &str = "performTime";
pub struct PerformTimePropertyIri;
impl PartialEq<&str> for PerformTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORM_TIME_PROPERTY_IRI_HTTP || *other == PERFORM_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PerformTimePropertyIri> for &str {
	fn eq(&self, other: &PerformTimePropertyIri) -> bool {
		*self == PERFORM_TIME_PROPERTY_IRI_HTTP || *self == PERFORM_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct PerformTimePropertyIriOrLabel;
impl PartialEq<&str> for PerformTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformTimePropertyIri || *other == PERFORM_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<PerformTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PerformTimePropertyIriOrLabel) -> bool {
		*self == PerformTimePropertyIri || *self == PERFORM_TIME_PROPERTY_LABEL
	}
}
