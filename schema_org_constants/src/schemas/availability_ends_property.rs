/// <https://schema.org/availabilityEnds>
pub const AVAILABILITY_ENDS_PROPERTY_IRI_HTTP: &str = "http://schema.org/availabilityEnds";
/// <https://schema.org/availabilityEnds>
pub const AVAILABILITY_ENDS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availabilityEnds";
/// <https://schema.org/availabilityEnds>
pub const AVAILABILITY_ENDS_PROPERTY_LABEL: &str = "availabilityEnds";
pub struct AvailabilityEndsPropertyIri;
impl PartialEq<&str> for AvailabilityEndsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABILITY_ENDS_PROPERTY_IRI_HTTP
			|| *other == AVAILABILITY_ENDS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailabilityEndsPropertyIri> for &str {
	fn eq(&self, other: &AvailabilityEndsPropertyIri) -> bool {
		*self == AVAILABILITY_ENDS_PROPERTY_IRI_HTTP
			|| *self == AVAILABILITY_ENDS_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailabilityEndsPropertyIriOrLabel;
impl PartialEq<&str> for AvailabilityEndsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailabilityEndsPropertyIri || *other == AVAILABILITY_ENDS_PROPERTY_LABEL
	}
}
impl PartialEq<AvailabilityEndsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailabilityEndsPropertyIriOrLabel) -> bool {
		*self == AvailabilityEndsPropertyIri || *self == AVAILABILITY_ENDS_PROPERTY_LABEL
	}
}
