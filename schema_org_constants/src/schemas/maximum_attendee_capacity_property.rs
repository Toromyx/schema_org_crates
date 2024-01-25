/// <https://schema.org/maximumAttendeeCapacity>
pub const MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/maximumAttendeeCapacity";
/// <https://schema.org/maximumAttendeeCapacity>
pub const MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/maximumAttendeeCapacity";
/// <https://schema.org/maximumAttendeeCapacity>
pub const MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_LABEL: &str = "maximumAttendeeCapacity";
pub struct MaximumAttendeeCapacityPropertyIri;
impl PartialEq<&str> for MaximumAttendeeCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *other == MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaximumAttendeeCapacityPropertyIri> for &str {
	fn eq(&self, other: &MaximumAttendeeCapacityPropertyIri) -> bool {
		*self == MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *self == MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct MaximumAttendeeCapacityPropertyIriOrLabel;
impl PartialEq<&str> for MaximumAttendeeCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaximumAttendeeCapacityPropertyIri
			|| *other == MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<MaximumAttendeeCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaximumAttendeeCapacityPropertyIriOrLabel) -> bool {
		*self == MaximumAttendeeCapacityPropertyIri
			|| *self == MAXIMUM_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
