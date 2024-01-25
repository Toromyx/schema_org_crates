/// <https://schema.org/UserDownloads>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_DOWNLOADS_IRI_HTTP: &str = "http://schema.org/UserDownloads";
/// <https://schema.org/UserDownloads>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_DOWNLOADS_IRI_HTTPS: &str = "https://schema.org/UserDownloads";
/// <https://schema.org/UserDownloads>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_DOWNLOADS_LABEL: &str = "UserDownloads";
pub struct UserDownloadsIri;
impl PartialEq<&str> for UserDownloadsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_DOWNLOADS_IRI_HTTP || *other == USER_DOWNLOADS_IRI_HTTPS
	}
}
impl PartialEq<UserDownloadsIri> for &str {
	fn eq(&self, other: &UserDownloadsIri) -> bool {
		*self == USER_DOWNLOADS_IRI_HTTP || *self == USER_DOWNLOADS_IRI_HTTPS
	}
}
pub struct UserDownloadsIriOrLabel;
impl PartialEq<&str> for UserDownloadsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserDownloadsIri || *other == USER_DOWNLOADS_LABEL
	}
}
impl PartialEq<UserDownloadsIriOrLabel> for &str {
	fn eq(&self, other: &UserDownloadsIriOrLabel) -> bool {
		*self == UserDownloadsIri || *self == USER_DOWNLOADS_LABEL
	}
}
