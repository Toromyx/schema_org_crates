/// <https://schema.org/RsvpResponseYes>
pub const RSVP_RESPONSE_YES_IRI_HTTP: &str = "http://schema.org/RsvpResponseYes";
/// <https://schema.org/RsvpResponseYes>
pub const RSVP_RESPONSE_YES_IRI_HTTPS: &str = "https://schema.org/RsvpResponseYes";
/// <https://schema.org/RsvpResponseYes>
pub const RSVP_RESPONSE_YES_LABEL: &str = "RsvpResponseYes";
pub struct RsvpResponseYesIri;
impl PartialEq<&str> for RsvpResponseYesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RSVP_RESPONSE_YES_IRI_HTTP || *other == RSVP_RESPONSE_YES_IRI_HTTPS
	}
}
impl PartialEq<RsvpResponseYesIri> for &str {
	fn eq(&self, other: &RsvpResponseYesIri) -> bool {
		*self == RSVP_RESPONSE_YES_IRI_HTTP || *self == RSVP_RESPONSE_YES_IRI_HTTPS
	}
}
pub struct RsvpResponseYesIriOrLabel;
impl PartialEq<&str> for RsvpResponseYesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RsvpResponseYesIri || *other == RSVP_RESPONSE_YES_LABEL
	}
}
impl PartialEq<RsvpResponseYesIriOrLabel> for &str {
	fn eq(&self, other: &RsvpResponseYesIriOrLabel) -> bool {
		*self == RsvpResponseYesIri || *self == RSVP_RESPONSE_YES_LABEL
	}
}
