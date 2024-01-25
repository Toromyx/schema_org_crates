/// <https://schema.org/availableThrough>
pub const AVAILABLE_THROUGH_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableThrough";
/// <https://schema.org/availableThrough>
pub const AVAILABLE_THROUGH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableThrough";
/// <https://schema.org/availableThrough>
pub const AVAILABLE_THROUGH_PROPERTY_LABEL: &str = "availableThrough";
pub struct AvailableThroughPropertyIri;
impl PartialEq<&str> for AvailableThroughPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_THROUGH_PROPERTY_IRI_HTTP
			|| *other == AVAILABLE_THROUGH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableThroughPropertyIri> for &str {
	fn eq(&self, other: &AvailableThroughPropertyIri) -> bool {
		*self == AVAILABLE_THROUGH_PROPERTY_IRI_HTTP
			|| *self == AVAILABLE_THROUGH_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableThroughPropertyIriOrLabel;
impl PartialEq<&str> for AvailableThroughPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableThroughPropertyIri || *other == AVAILABLE_THROUGH_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableThroughPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableThroughPropertyIriOrLabel) -> bool {
		*self == AvailableThroughPropertyIri || *self == AVAILABLE_THROUGH_PROPERTY_LABEL
	}
}
