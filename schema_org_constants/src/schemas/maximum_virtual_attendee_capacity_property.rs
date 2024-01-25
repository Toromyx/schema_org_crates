/// <https://schema.org/maximumVirtualAttendeeCapacity>
pub const MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/maximumVirtualAttendeeCapacity";
/// <https://schema.org/maximumVirtualAttendeeCapacity>
pub const MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/maximumVirtualAttendeeCapacity";
/// <https://schema.org/maximumVirtualAttendeeCapacity>
pub const MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_LABEL: &str = "maximumVirtualAttendeeCapacity";
pub struct MaximumVirtualAttendeeCapacityPropertyIri;
impl PartialEq<&str> for MaximumVirtualAttendeeCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *other == MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaximumVirtualAttendeeCapacityPropertyIri> for &str {
	fn eq(&self, other: &MaximumVirtualAttendeeCapacityPropertyIri) -> bool {
		*self == MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *self == MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct MaximumVirtualAttendeeCapacityPropertyIriOrLabel;
impl PartialEq<&str> for MaximumVirtualAttendeeCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaximumVirtualAttendeeCapacityPropertyIri
			|| *other == MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<MaximumVirtualAttendeeCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaximumVirtualAttendeeCapacityPropertyIriOrLabel) -> bool {
		*self == MaximumVirtualAttendeeCapacityPropertyIri
			|| *self == MAXIMUM_VIRTUAL_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
