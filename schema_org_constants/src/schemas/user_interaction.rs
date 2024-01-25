/// <https://schema.org/UserInteraction>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_INTERACTION_IRI_HTTP: &str = "http://schema.org/UserInteraction";
/// <https://schema.org/UserInteraction>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_INTERACTION_IRI_HTTPS: &str = "https://schema.org/UserInteraction";
/// <https://schema.org/UserInteraction>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_INTERACTION_LABEL: &str = "UserInteraction";
pub struct UserInteractionIri;
impl PartialEq<&str> for UserInteractionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_INTERACTION_IRI_HTTP || *other == USER_INTERACTION_IRI_HTTPS
	}
}
impl PartialEq<UserInteractionIri> for &str {
	fn eq(&self, other: &UserInteractionIri) -> bool {
		*self == USER_INTERACTION_IRI_HTTP || *self == USER_INTERACTION_IRI_HTTPS
	}
}
pub struct UserInteractionIriOrLabel;
impl PartialEq<&str> for UserInteractionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserInteractionIri || *other == USER_INTERACTION_LABEL
	}
}
impl PartialEq<UserInteractionIriOrLabel> for &str {
	fn eq(&self, other: &UserInteractionIriOrLabel) -> bool {
		*self == UserInteractionIri || *self == USER_INTERACTION_LABEL
	}
}
