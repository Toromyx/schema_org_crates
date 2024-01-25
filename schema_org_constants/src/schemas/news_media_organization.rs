/// <https://schema.org/NewsMediaOrganization>
pub const NEWS_MEDIA_ORGANIZATION_IRI_HTTP: &str = "http://schema.org/NewsMediaOrganization";
/// <https://schema.org/NewsMediaOrganization>
pub const NEWS_MEDIA_ORGANIZATION_IRI_HTTPS: &str = "https://schema.org/NewsMediaOrganization";
/// <https://schema.org/NewsMediaOrganization>
pub const NEWS_MEDIA_ORGANIZATION_LABEL: &str = "NewsMediaOrganization";
pub struct NewsMediaOrganizationIri;
impl PartialEq<&str> for NewsMediaOrganizationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEWS_MEDIA_ORGANIZATION_IRI_HTTP || *other == NEWS_MEDIA_ORGANIZATION_IRI_HTTPS
	}
}
impl PartialEq<NewsMediaOrganizationIri> for &str {
	fn eq(&self, other: &NewsMediaOrganizationIri) -> bool {
		*self == NEWS_MEDIA_ORGANIZATION_IRI_HTTP || *self == NEWS_MEDIA_ORGANIZATION_IRI_HTTPS
	}
}
pub struct NewsMediaOrganizationIriOrLabel;
impl PartialEq<&str> for NewsMediaOrganizationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NewsMediaOrganizationIri || *other == NEWS_MEDIA_ORGANIZATION_LABEL
	}
}
impl PartialEq<NewsMediaOrganizationIriOrLabel> for &str {
	fn eq(&self, other: &NewsMediaOrganizationIriOrLabel) -> bool {
		*self == NewsMediaOrganizationIri || *self == NEWS_MEDIA_ORGANIZATION_LABEL
	}
}
