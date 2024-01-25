/// <https://schema.org/UserCheckins>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_CHECKINS_IRI_HTTP: &str = "http://schema.org/UserCheckins";
/// <https://schema.org/UserCheckins>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_CHECKINS_IRI_HTTPS: &str = "https://schema.org/UserCheckins";
/// <https://schema.org/UserCheckins>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_CHECKINS_LABEL: &str = "UserCheckins";
pub struct UserCheckinsIri;
impl PartialEq<&str> for UserCheckinsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_CHECKINS_IRI_HTTP || *other == USER_CHECKINS_IRI_HTTPS
	}
}
impl PartialEq<UserCheckinsIri> for &str {
	fn eq(&self, other: &UserCheckinsIri) -> bool {
		*self == USER_CHECKINS_IRI_HTTP || *self == USER_CHECKINS_IRI_HTTPS
	}
}
pub struct UserCheckinsIriOrLabel;
impl PartialEq<&str> for UserCheckinsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserCheckinsIri || *other == USER_CHECKINS_LABEL
	}
}
impl PartialEq<UserCheckinsIriOrLabel> for &str {
	fn eq(&self, other: &UserCheckinsIriOrLabel) -> bool {
		*self == UserCheckinsIri || *self == USER_CHECKINS_LABEL
	}
}
