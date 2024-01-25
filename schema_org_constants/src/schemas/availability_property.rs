/// <https://schema.org/availability>
pub const AVAILABILITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/availability";
/// <https://schema.org/availability>
pub const AVAILABILITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availability";
/// <https://schema.org/availability>
pub const AVAILABILITY_PROPERTY_LABEL: &str = "availability";
pub struct AvailabilityPropertyIri;
impl PartialEq<&str> for AvailabilityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABILITY_PROPERTY_IRI_HTTP || *other == AVAILABILITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailabilityPropertyIri> for &str {
	fn eq(&self, other: &AvailabilityPropertyIri) -> bool {
		*self == AVAILABILITY_PROPERTY_IRI_HTTP || *self == AVAILABILITY_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailabilityPropertyIriOrLabel;
impl PartialEq<&str> for AvailabilityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailabilityPropertyIri || *other == AVAILABILITY_PROPERTY_LABEL
	}
}
impl PartialEq<AvailabilityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailabilityPropertyIriOrLabel) -> bool {
		*self == AvailabilityPropertyIri || *self == AVAILABILITY_PROPERTY_LABEL
	}
}
