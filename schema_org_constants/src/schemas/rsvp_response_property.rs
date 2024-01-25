/// <https://schema.org/rsvpResponse>
pub const RSVP_RESPONSE_PROPERTY_IRI_HTTP: &str = "http://schema.org/rsvpResponse";
/// <https://schema.org/rsvpResponse>
pub const RSVP_RESPONSE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/rsvpResponse";
/// <https://schema.org/rsvpResponse>
pub const RSVP_RESPONSE_PROPERTY_LABEL: &str = "rsvpResponse";
pub struct RsvpResponsePropertyIri;
impl PartialEq<&str> for RsvpResponsePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RSVP_RESPONSE_PROPERTY_IRI_HTTP || *other == RSVP_RESPONSE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RsvpResponsePropertyIri> for &str {
	fn eq(&self, other: &RsvpResponsePropertyIri) -> bool {
		*self == RSVP_RESPONSE_PROPERTY_IRI_HTTP || *self == RSVP_RESPONSE_PROPERTY_IRI_HTTPS
	}
}
pub struct RsvpResponsePropertyIriOrLabel;
impl PartialEq<&str> for RsvpResponsePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RsvpResponsePropertyIri || *other == RSVP_RESPONSE_PROPERTY_LABEL
	}
}
impl PartialEq<RsvpResponsePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RsvpResponsePropertyIriOrLabel) -> bool {
		*self == RsvpResponsePropertyIri || *self == RSVP_RESPONSE_PROPERTY_LABEL
	}
}
