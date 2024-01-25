/// <https://schema.org/availableFrom>
pub const AVAILABLE_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableFrom";
/// <https://schema.org/availableFrom>
pub const AVAILABLE_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableFrom";
/// <https://schema.org/availableFrom>
pub const AVAILABLE_FROM_PROPERTY_LABEL: &str = "availableFrom";
pub struct AvailableFromPropertyIri;
impl PartialEq<&str> for AvailableFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_FROM_PROPERTY_IRI_HTTP || *other == AVAILABLE_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableFromPropertyIri> for &str {
	fn eq(&self, other: &AvailableFromPropertyIri) -> bool {
		*self == AVAILABLE_FROM_PROPERTY_IRI_HTTP || *self == AVAILABLE_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableFromPropertyIriOrLabel;
impl PartialEq<&str> for AvailableFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableFromPropertyIri || *other == AVAILABLE_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableFromPropertyIriOrLabel) -> bool {
		*self == AvailableFromPropertyIri || *self == AVAILABLE_FROM_PROPERTY_LABEL
	}
}
