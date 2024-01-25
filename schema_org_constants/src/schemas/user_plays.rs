/// <https://schema.org/UserPlays>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PLAYS_IRI_HTTP: &str = "http://schema.org/UserPlays";
/// <https://schema.org/UserPlays>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PLAYS_IRI_HTTPS: &str = "https://schema.org/UserPlays";
/// <https://schema.org/UserPlays>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PLAYS_LABEL: &str = "UserPlays";
pub struct UserPlaysIri;
impl PartialEq<&str> for UserPlaysIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_PLAYS_IRI_HTTP || *other == USER_PLAYS_IRI_HTTPS
	}
}
impl PartialEq<UserPlaysIri> for &str {
	fn eq(&self, other: &UserPlaysIri) -> bool {
		*self == USER_PLAYS_IRI_HTTP || *self == USER_PLAYS_IRI_HTTPS
	}
}
pub struct UserPlaysIriOrLabel;
impl PartialEq<&str> for UserPlaysIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserPlaysIri || *other == USER_PLAYS_LABEL
	}
}
impl PartialEq<UserPlaysIriOrLabel> for &str {
	fn eq(&self, other: &UserPlaysIriOrLabel) -> bool {
		*self == UserPlaysIri || *self == USER_PLAYS_LABEL
	}
}
