/// <https://schema.org/RsvpResponseType>
pub const RSVP_RESPONSE_TYPE_IRI_HTTP: &str = "http://schema.org/RsvpResponseType";
/// <https://schema.org/RsvpResponseType>
pub const RSVP_RESPONSE_TYPE_IRI_HTTPS: &str = "https://schema.org/RsvpResponseType";
/// <https://schema.org/RsvpResponseType>
pub const RSVP_RESPONSE_TYPE_LABEL: &str = "RsvpResponseType";
pub struct RsvpResponseTypeIri;
impl PartialEq<&str> for RsvpResponseTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RSVP_RESPONSE_TYPE_IRI_HTTP || *other == RSVP_RESPONSE_TYPE_IRI_HTTPS
	}
}
impl PartialEq<RsvpResponseTypeIri> for &str {
	fn eq(&self, other: &RsvpResponseTypeIri) -> bool {
		*self == RSVP_RESPONSE_TYPE_IRI_HTTP || *self == RSVP_RESPONSE_TYPE_IRI_HTTPS
	}
}
pub struct RsvpResponseTypeIriOrLabel;
impl PartialEq<&str> for RsvpResponseTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RsvpResponseTypeIri || *other == RSVP_RESPONSE_TYPE_LABEL
	}
}
impl PartialEq<RsvpResponseTypeIriOrLabel> for &str {
	fn eq(&self, other: &RsvpResponseTypeIriOrLabel) -> bool {
		*self == RsvpResponseTypeIri || *self == RSVP_RESPONSE_TYPE_LABEL
	}
}
