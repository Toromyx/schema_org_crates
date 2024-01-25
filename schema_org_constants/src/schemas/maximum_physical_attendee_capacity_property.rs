/// <https://schema.org/maximumPhysicalAttendeeCapacity>
pub const MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/maximumPhysicalAttendeeCapacity";
/// <https://schema.org/maximumPhysicalAttendeeCapacity>
pub const MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/maximumPhysicalAttendeeCapacity";
/// <https://schema.org/maximumPhysicalAttendeeCapacity>
pub const MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_LABEL: &str =
	"maximumPhysicalAttendeeCapacity";
pub struct MaximumPhysicalAttendeeCapacityPropertyIri;
impl PartialEq<&str> for MaximumPhysicalAttendeeCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *other == MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaximumPhysicalAttendeeCapacityPropertyIri> for &str {
	fn eq(&self, other: &MaximumPhysicalAttendeeCapacityPropertyIri) -> bool {
		*self == MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *self == MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct MaximumPhysicalAttendeeCapacityPropertyIriOrLabel;
impl PartialEq<&str> for MaximumPhysicalAttendeeCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaximumPhysicalAttendeeCapacityPropertyIri
			|| *other == MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<MaximumPhysicalAttendeeCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaximumPhysicalAttendeeCapacityPropertyIriOrLabel) -> bool {
		*self == MaximumPhysicalAttendeeCapacityPropertyIri
			|| *self == MAXIMUM_PHYSICAL_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
