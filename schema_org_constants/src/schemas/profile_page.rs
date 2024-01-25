/// <https://schema.org/ProfilePage>
pub const PROFILE_PAGE_IRI_HTTP: &str = "http://schema.org/ProfilePage";
/// <https://schema.org/ProfilePage>
pub const PROFILE_PAGE_IRI_HTTPS: &str = "https://schema.org/ProfilePage";
/// <https://schema.org/ProfilePage>
pub const PROFILE_PAGE_LABEL: &str = "ProfilePage";
pub struct ProfilePageIri;
impl PartialEq<&str> for ProfilePageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROFILE_PAGE_IRI_HTTP || *other == PROFILE_PAGE_IRI_HTTPS
	}
}
impl PartialEq<ProfilePageIri> for &str {
	fn eq(&self, other: &ProfilePageIri) -> bool {
		*self == PROFILE_PAGE_IRI_HTTP || *self == PROFILE_PAGE_IRI_HTTPS
	}
}
pub struct ProfilePageIriOrLabel;
impl PartialEq<&str> for ProfilePageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProfilePageIri || *other == PROFILE_PAGE_LABEL
	}
}
impl PartialEq<ProfilePageIriOrLabel> for &str {
	fn eq(&self, other: &ProfilePageIriOrLabel) -> bool {
		*self == ProfilePageIri || *self == PROFILE_PAGE_LABEL
	}
}
