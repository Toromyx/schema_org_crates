/// <https://schema.org/attendee>
pub const ATTENDEE_PROPERTY_IRI_HTTP: &str = "http://schema.org/attendee";
/// <https://schema.org/attendee>
pub const ATTENDEE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/attendee";
/// <https://schema.org/attendee>
pub const ATTENDEE_PROPERTY_LABEL: &str = "attendee";
pub struct AttendeePropertyIri;
impl PartialEq<&str> for AttendeePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ATTENDEE_PROPERTY_IRI_HTTP || *other == ATTENDEE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AttendeePropertyIri> for &str {
	fn eq(&self, other: &AttendeePropertyIri) -> bool {
		*self == ATTENDEE_PROPERTY_IRI_HTTP || *self == ATTENDEE_PROPERTY_IRI_HTTPS
	}
}
pub struct AttendeePropertyIriOrLabel;
impl PartialEq<&str> for AttendeePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AttendeePropertyIri || *other == ATTENDEE_PROPERTY_LABEL
	}
}
impl PartialEq<AttendeePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AttendeePropertyIriOrLabel) -> bool {
		*self == AttendeePropertyIri || *self == ATTENDEE_PROPERTY_LABEL
	}
}
