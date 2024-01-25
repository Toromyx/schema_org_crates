/// <https://schema.org/remainingAttendeeCapacity>
pub const REMAINING_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/remainingAttendeeCapacity";
/// <https://schema.org/remainingAttendeeCapacity>
pub const REMAINING_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/remainingAttendeeCapacity";
/// <https://schema.org/remainingAttendeeCapacity>
pub const REMAINING_ATTENDEE_CAPACITY_PROPERTY_LABEL: &str = "remainingAttendeeCapacity";
pub struct RemainingAttendeeCapacityPropertyIri;
impl PartialEq<&str> for RemainingAttendeeCapacityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REMAINING_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *other == REMAINING_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RemainingAttendeeCapacityPropertyIri> for &str {
	fn eq(&self, other: &RemainingAttendeeCapacityPropertyIri) -> bool {
		*self == REMAINING_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTP
			|| *self == REMAINING_ATTENDEE_CAPACITY_PROPERTY_IRI_HTTPS
	}
}
pub struct RemainingAttendeeCapacityPropertyIriOrLabel;
impl PartialEq<&str> for RemainingAttendeeCapacityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RemainingAttendeeCapacityPropertyIri
			|| *other == REMAINING_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
impl PartialEq<RemainingAttendeeCapacityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RemainingAttendeeCapacityPropertyIriOrLabel) -> bool {
		*self == RemainingAttendeeCapacityPropertyIri
			|| *self == REMAINING_ATTENDEE_CAPACITY_PROPERTY_LABEL
	}
}
