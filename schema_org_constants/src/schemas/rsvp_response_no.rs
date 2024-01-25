/// <https://schema.org/RsvpResponseNo>
pub const RSVP_RESPONSE_NO_IRI_HTTP: &str = "http://schema.org/RsvpResponseNo";
/// <https://schema.org/RsvpResponseNo>
pub const RSVP_RESPONSE_NO_IRI_HTTPS: &str = "https://schema.org/RsvpResponseNo";
/// <https://schema.org/RsvpResponseNo>
pub const RSVP_RESPONSE_NO_LABEL: &str = "RsvpResponseNo";
pub struct RsvpResponseNoIri;
impl PartialEq<&str> for RsvpResponseNoIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RSVP_RESPONSE_NO_IRI_HTTP || *other == RSVP_RESPONSE_NO_IRI_HTTPS
	}
}
impl PartialEq<RsvpResponseNoIri> for &str {
	fn eq(&self, other: &RsvpResponseNoIri) -> bool {
		*self == RSVP_RESPONSE_NO_IRI_HTTP || *self == RSVP_RESPONSE_NO_IRI_HTTPS
	}
}
pub struct RsvpResponseNoIriOrLabel;
impl PartialEq<&str> for RsvpResponseNoIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RsvpResponseNoIri || *other == RSVP_RESPONSE_NO_LABEL
	}
}
impl PartialEq<RsvpResponseNoIriOrLabel> for &str {
	fn eq(&self, other: &RsvpResponseNoIriOrLabel) -> bool {
		*self == RsvpResponseNoIri || *self == RSVP_RESPONSE_NO_LABEL
	}
}
