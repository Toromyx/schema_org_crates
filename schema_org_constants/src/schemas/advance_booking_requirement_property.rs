/// <https://schema.org/advanceBookingRequirement>
pub const ADVANCE_BOOKING_REQUIREMENT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/advanceBookingRequirement";
/// <https://schema.org/advanceBookingRequirement>
pub const ADVANCE_BOOKING_REQUIREMENT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/advanceBookingRequirement";
/// <https://schema.org/advanceBookingRequirement>
pub const ADVANCE_BOOKING_REQUIREMENT_PROPERTY_LABEL: &str = "advanceBookingRequirement";
pub struct AdvanceBookingRequirementPropertyIri;
impl PartialEq<&str> for AdvanceBookingRequirementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADVANCE_BOOKING_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *other == ADVANCE_BOOKING_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AdvanceBookingRequirementPropertyIri> for &str {
	fn eq(&self, other: &AdvanceBookingRequirementPropertyIri) -> bool {
		*self == ADVANCE_BOOKING_REQUIREMENT_PROPERTY_IRI_HTTP
			|| *self == ADVANCE_BOOKING_REQUIREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct AdvanceBookingRequirementPropertyIriOrLabel;
impl PartialEq<&str> for AdvanceBookingRequirementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdvanceBookingRequirementPropertyIri
			|| *other == ADVANCE_BOOKING_REQUIREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<AdvanceBookingRequirementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AdvanceBookingRequirementPropertyIriOrLabel) -> bool {
		*self == AdvanceBookingRequirementPropertyIri
			|| *self == ADVANCE_BOOKING_REQUIREMENT_PROPERTY_LABEL
	}
}
