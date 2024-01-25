/// <https://schema.org/RsvpAction>
pub const RSVP_ACTION_IRI_HTTP: &str = "http://schema.org/RsvpAction";
/// <https://schema.org/RsvpAction>
pub const RSVP_ACTION_IRI_HTTPS: &str = "https://schema.org/RsvpAction";
/// <https://schema.org/RsvpAction>
pub const RSVP_ACTION_LABEL: &str = "RsvpAction";
pub struct RsvpActionIri;
impl PartialEq<&str> for RsvpActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RSVP_ACTION_IRI_HTTP || *other == RSVP_ACTION_IRI_HTTPS
	}
}
impl PartialEq<RsvpActionIri> for &str {
	fn eq(&self, other: &RsvpActionIri) -> bool {
		*self == RSVP_ACTION_IRI_HTTP || *self == RSVP_ACTION_IRI_HTTPS
	}
}
pub struct RsvpActionIriOrLabel;
impl PartialEq<&str> for RsvpActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RsvpActionIri || *other == RSVP_ACTION_LABEL
	}
}
impl PartialEq<RsvpActionIriOrLabel> for &str {
	fn eq(&self, other: &RsvpActionIriOrLabel) -> bool {
		*self == RsvpActionIri || *self == RSVP_ACTION_LABEL
	}
}
