/// <https://schema.org/UserPageVisits>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PAGE_VISITS_IRI_HTTP: &str = "http://schema.org/UserPageVisits";
/// <https://schema.org/UserPageVisits>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PAGE_VISITS_IRI_HTTPS: &str = "https://schema.org/UserPageVisits";
/// <https://schema.org/UserPageVisits>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_PAGE_VISITS_LABEL: &str = "UserPageVisits";
pub struct UserPageVisitsIri;
impl PartialEq<&str> for UserPageVisitsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_PAGE_VISITS_IRI_HTTP || *other == USER_PAGE_VISITS_IRI_HTTPS
	}
}
impl PartialEq<UserPageVisitsIri> for &str {
	fn eq(&self, other: &UserPageVisitsIri) -> bool {
		*self == USER_PAGE_VISITS_IRI_HTTP || *self == USER_PAGE_VISITS_IRI_HTTPS
	}
}
pub struct UserPageVisitsIriOrLabel;
impl PartialEq<&str> for UserPageVisitsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserPageVisitsIri || *other == USER_PAGE_VISITS_LABEL
	}
}
impl PartialEq<UserPageVisitsIriOrLabel> for &str {
	fn eq(&self, other: &UserPageVisitsIriOrLabel) -> bool {
		*self == UserPageVisitsIri || *self == USER_PAGE_VISITS_LABEL
	}
}
