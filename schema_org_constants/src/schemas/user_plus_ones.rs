/// <https://schema.org/UserPlusOnes>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PLUS_ONES_IRI_HTTP: &str = "http://schema.org/UserPlusOnes";
/// <https://schema.org/UserPlusOnes>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PLUS_ONES_IRI_HTTPS: &str = "https://schema.org/UserPlusOnes";
/// <https://schema.org/UserPlusOnes>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PLUS_ONES_LABEL: &str = "UserPlusOnes";
pub struct UserPlusOnesIri;
impl PartialEq<&str> for UserPlusOnesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_PLUS_ONES_IRI_HTTP || *other == USER_PLUS_ONES_IRI_HTTPS
	}
}
impl PartialEq<UserPlusOnesIri> for &str {
	fn eq(&self, other: &UserPlusOnesIri) -> bool {
		*self == USER_PLUS_ONES_IRI_HTTP || *self == USER_PLUS_ONES_IRI_HTTPS
	}
}
pub struct UserPlusOnesIriOrLabel;
impl PartialEq<&str> for UserPlusOnesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserPlusOnesIri || *other == USER_PLUS_ONES_LABEL
	}
}
impl PartialEq<UserPlusOnesIriOrLabel> for &str {
	fn eq(&self, other: &UserPlusOnesIriOrLabel) -> bool {
		*self == UserPlusOnesIri || *self == USER_PLUS_ONES_LABEL
	}
}
