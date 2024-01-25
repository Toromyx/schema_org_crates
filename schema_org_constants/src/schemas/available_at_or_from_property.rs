/// <https://schema.org/availableAtOrFrom>
pub const AVAILABLE_AT_OR_FROM_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableAtOrFrom";
/// <https://schema.org/availableAtOrFrom>
pub const AVAILABLE_AT_OR_FROM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableAtOrFrom";
/// <https://schema.org/availableAtOrFrom>
pub const AVAILABLE_AT_OR_FROM_PROPERTY_LABEL: &str = "availableAtOrFrom";
pub struct AvailableAtOrFromPropertyIri;
impl PartialEq<&str> for AvailableAtOrFromPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_AT_OR_FROM_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_AT_OR_FROM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableAtOrFromPropertyIri> for &str {
	fn eq(&self, other: &AvailableAtOrFromPropertyIri) -> bool {
		*self == AVAILABLE_AT_OR_FROM_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_AT_OR_FROM_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableAtOrFromPropertyIriOrLabel;
impl PartialEq<&str> for AvailableAtOrFromPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableAtOrFromPropertyIri || *other == AVAILABLE_AT_OR_FROM_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableAtOrFromPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableAtOrFromPropertyIriOrLabel) -> bool {
		*self == AvailableAtOrFromPropertyIri || *self == AVAILABLE_AT_OR_FROM_PROPERTY_LABEL
	}
}
