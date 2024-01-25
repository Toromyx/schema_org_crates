/// <https://schema.org/UserLikes>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_LIKES_IRI_HTTP: &str = "http://schema.org/UserLikes";
/// <https://schema.org/UserLikes>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_LIKES_IRI_HTTPS: &str = "https://schema.org/UserLikes";
/// <https://schema.org/UserLikes>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_LIKES_LABEL: &str = "UserLikes";
pub struct UserLikesIri;
impl PartialEq<&str> for UserLikesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_LIKES_IRI_HTTP || *other == USER_LIKES_IRI_HTTPS
	}
}
impl PartialEq<UserLikesIri> for &str {
	fn eq(&self, other: &UserLikesIri) -> bool {
		*self == USER_LIKES_IRI_HTTP || *self == USER_LIKES_IRI_HTTPS
	}
}
pub struct UserLikesIriOrLabel;
impl PartialEq<&str> for UserLikesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserLikesIri || *other == USER_LIKES_LABEL
	}
}
impl PartialEq<UserLikesIriOrLabel> for &str {
	fn eq(&self, other: &UserLikesIriOrLabel) -> bool {
		*self == UserLikesIri || *self == USER_LIKES_LABEL
	}
}
