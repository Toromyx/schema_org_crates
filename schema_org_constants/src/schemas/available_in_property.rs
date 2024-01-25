/// <https://schema.org/availableIn>
pub const AVAILABLE_IN_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableIn";
/// <https://schema.org/availableIn>
pub const AVAILABLE_IN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableIn";
/// <https://schema.org/availableIn>
pub const AVAILABLE_IN_PROPERTY_LABEL: &str = "availableIn";
pub struct AvailableInPropertyIri;
impl PartialEq<&str> for AvailableInPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_IN_PROPERTY_IRI_HTTP || *other == AVAILABLE_IN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableInPropertyIri> for &str {
	fn eq(&self, other: &AvailableInPropertyIri) -> bool {
		*self == AVAILABLE_IN_PROPERTY_IRI_HTTP || *self == AVAILABLE_IN_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableInPropertyIriOrLabel;
impl PartialEq<&str> for AvailableInPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableInPropertyIri || *other == AVAILABLE_IN_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableInPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableInPropertyIriOrLabel) -> bool {
		*self == AvailableInPropertyIri || *self == AVAILABLE_IN_PROPERTY_LABEL
	}
}
