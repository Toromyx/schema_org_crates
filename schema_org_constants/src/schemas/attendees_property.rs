/// <https://schema.org/attendees>
#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
pub const ATTENDEES_PROPERTY_IRI_HTTP: &str = "http://schema.org/attendees";
/// <https://schema.org/attendees>
#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
pub const ATTENDEES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/attendees";
/// <https://schema.org/attendees>
#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
pub const ATTENDEES_PROPERTY_LABEL: &str = "attendees";
pub struct AttendeesPropertyIri;
impl PartialEq<&str> for AttendeesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ATTENDEES_PROPERTY_IRI_HTTP || *other == ATTENDEES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AttendeesPropertyIri> for &str {
	fn eq(&self, other: &AttendeesPropertyIri) -> bool {
		*self == ATTENDEES_PROPERTY_IRI_HTTP || *self == ATTENDEES_PROPERTY_IRI_HTTPS
	}
}
pub struct AttendeesPropertyIriOrLabel;
impl PartialEq<&str> for AttendeesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AttendeesPropertyIri || *other == ATTENDEES_PROPERTY_LABEL
	}
}
impl PartialEq<AttendeesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AttendeesPropertyIriOrLabel) -> bool {
		*self == AttendeesPropertyIri || *self == ATTENDEES_PROPERTY_LABEL
	}
}
