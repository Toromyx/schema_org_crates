/// <https://schema.org/UserComments>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_COMMENTS_IRI_HTTP: &str = "http://schema.org/UserComments";
/// <https://schema.org/UserComments>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_COMMENTS_IRI_HTTPS: &str = "https://schema.org/UserComments";
/// <https://schema.org/UserComments>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_COMMENTS_LABEL: &str = "UserComments";
pub struct UserCommentsIri;
impl PartialEq<&str> for UserCommentsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_COMMENTS_IRI_HTTP || *other == USER_COMMENTS_IRI_HTTPS
	}
}
impl PartialEq<UserCommentsIri> for &str {
	fn eq(&self, other: &UserCommentsIri) -> bool {
		*self == USER_COMMENTS_IRI_HTTP || *self == USER_COMMENTS_IRI_HTTPS
	}
}
pub struct UserCommentsIriOrLabel;
impl PartialEq<&str> for UserCommentsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserCommentsIri || *other == USER_COMMENTS_LABEL
	}
}
impl PartialEq<UserCommentsIriOrLabel> for &str {
	fn eq(&self, other: &UserCommentsIriOrLabel) -> bool {
		*self == UserCommentsIri || *self == USER_COMMENTS_LABEL
	}
}
