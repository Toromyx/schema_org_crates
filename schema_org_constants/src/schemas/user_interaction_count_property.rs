/// <https://schema.org/userInteractionCount>
pub const USER_INTERACTION_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/userInteractionCount";
/// <https://schema.org/userInteractionCount>
pub const USER_INTERACTION_COUNT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/userInteractionCount";
/// <https://schema.org/userInteractionCount>
pub const USER_INTERACTION_COUNT_PROPERTY_LABEL: &str = "userInteractionCount";
pub struct UserInteractionCountPropertyIri;
impl PartialEq<&str> for UserInteractionCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_INTERACTION_COUNT_PROPERTY_IRI_HTTP
			|| *other == USER_INTERACTION_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UserInteractionCountPropertyIri> for &str {
	fn eq(&self, other: &UserInteractionCountPropertyIri) -> bool {
		*self == USER_INTERACTION_COUNT_PROPERTY_IRI_HTTP
			|| *self == USER_INTERACTION_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct UserInteractionCountPropertyIriOrLabel;
impl PartialEq<&str> for UserInteractionCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserInteractionCountPropertyIri || *other == USER_INTERACTION_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<UserInteractionCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UserInteractionCountPropertyIriOrLabel) -> bool {
		*self == UserInteractionCountPropertyIri || *self == USER_INTERACTION_COUNT_PROPERTY_LABEL
	}
}
