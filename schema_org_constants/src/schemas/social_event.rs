/// <https://schema.org/SocialEvent>
pub const SOCIAL_EVENT_IRI_HTTP: &str = "http://schema.org/SocialEvent";
/// <https://schema.org/SocialEvent>
pub const SOCIAL_EVENT_IRI_HTTPS: &str = "https://schema.org/SocialEvent";
/// <https://schema.org/SocialEvent>
pub const SOCIAL_EVENT_LABEL: &str = "SocialEvent";
pub struct SocialEventIri;
impl PartialEq<&str> for SocialEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOCIAL_EVENT_IRI_HTTP || *other == SOCIAL_EVENT_IRI_HTTPS
	}
}
impl PartialEq<SocialEventIri> for &str {
	fn eq(&self, other: &SocialEventIri) -> bool {
		*self == SOCIAL_EVENT_IRI_HTTP || *self == SOCIAL_EVENT_IRI_HTTPS
	}
}
pub struct SocialEventIriOrLabel;
impl PartialEq<&str> for SocialEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SocialEventIri || *other == SOCIAL_EVENT_LABEL
	}
}
impl PartialEq<SocialEventIriOrLabel> for &str {
	fn eq(&self, other: &SocialEventIriOrLabel) -> bool {
		*self == SocialEventIri || *self == SOCIAL_EVENT_LABEL
	}
}
