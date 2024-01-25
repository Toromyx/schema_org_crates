/// <https://schema.org/RsvpResponseMaybe>
pub const RSVP_RESPONSE_MAYBE_IRI_HTTP: &str = "http://schema.org/RsvpResponseMaybe";
/// <https://schema.org/RsvpResponseMaybe>
pub const RSVP_RESPONSE_MAYBE_IRI_HTTPS: &str = "https://schema.org/RsvpResponseMaybe";
/// <https://schema.org/RsvpResponseMaybe>
pub const RSVP_RESPONSE_MAYBE_LABEL: &str = "RsvpResponseMaybe";
pub struct RsvpResponseMaybeIri;
impl PartialEq<&str> for RsvpResponseMaybeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RSVP_RESPONSE_MAYBE_IRI_HTTP || *other == RSVP_RESPONSE_MAYBE_IRI_HTTPS
	}
}
impl PartialEq<RsvpResponseMaybeIri> for &str {
	fn eq(&self, other: &RsvpResponseMaybeIri) -> bool {
		*self == RSVP_RESPONSE_MAYBE_IRI_HTTP || *self == RSVP_RESPONSE_MAYBE_IRI_HTTPS
	}
}
pub struct RsvpResponseMaybeIriOrLabel;
impl PartialEq<&str> for RsvpResponseMaybeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RsvpResponseMaybeIri || *other == RSVP_RESPONSE_MAYBE_LABEL
	}
}
impl PartialEq<RsvpResponseMaybeIriOrLabel> for &str {
	fn eq(&self, other: &RsvpResponseMaybeIriOrLabel) -> bool {
		*self == RsvpResponseMaybeIri || *self == RSVP_RESPONSE_MAYBE_LABEL
	}
}
