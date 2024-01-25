/// <https://schema.org/availabilityStarts>
pub const AVAILABILITY_STARTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/availabilityStarts";
/// <https://schema.org/availabilityStarts>
pub const AVAILABILITY_STARTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availabilityStarts";
/// <https://schema.org/availabilityStarts>
pub const AVAILABILITY_STARTS_PROPERTY_LABEL: &str = "availabilityStarts";
pub struct AvailabilityStartsPropertyIri;
impl PartialEq<&str> for AvailabilityStartsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABILITY_STARTS_PROPERTY_IRI_HTTP
			|| *other == AVAILABILITY_STARTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailabilityStartsPropertyIri> for &str {
	fn eq(&self, other: &AvailabilityStartsPropertyIri) -> bool {
		*self == AVAILABILITY_STARTS_PROPERTY_IRI_HTTP
			|| *self == AVAILABILITY_STARTS_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailabilityStartsPropertyIriOrLabel;
impl PartialEq<&str> for AvailabilityStartsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailabilityStartsPropertyIri || *other == AVAILABILITY_STARTS_PROPERTY_LABEL
	}
}
impl PartialEq<AvailabilityStartsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailabilityStartsPropertyIriOrLabel) -> bool {
		*self == AvailabilityStartsPropertyIri || *self == AVAILABILITY_STARTS_PROPERTY_LABEL
	}
}
