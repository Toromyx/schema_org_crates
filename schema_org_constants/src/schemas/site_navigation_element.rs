/// <https://schema.org/SiteNavigationElement>
pub const SITE_NAVIGATION_ELEMENT_IRI_HTTP: &str = "http://schema.org/SiteNavigationElement";
/// <https://schema.org/SiteNavigationElement>
pub const SITE_NAVIGATION_ELEMENT_IRI_HTTPS: &str = "https://schema.org/SiteNavigationElement";
/// <https://schema.org/SiteNavigationElement>
pub const SITE_NAVIGATION_ELEMENT_LABEL: &str = "SiteNavigationElement";
pub struct SiteNavigationElementIri;
impl PartialEq<&str> for SiteNavigationElementIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SITE_NAVIGATION_ELEMENT_IRI_HTTP || *other == SITE_NAVIGATION_ELEMENT_IRI_HTTPS
	}
}
impl PartialEq<SiteNavigationElementIri> for &str {
	fn eq(&self, other: &SiteNavigationElementIri) -> bool {
		*self == SITE_NAVIGATION_ELEMENT_IRI_HTTP || *self == SITE_NAVIGATION_ELEMENT_IRI_HTTPS
	}
}
pub struct SiteNavigationElementIriOrLabel;
impl PartialEq<&str> for SiteNavigationElementIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SiteNavigationElementIri || *other == SITE_NAVIGATION_ELEMENT_LABEL
	}
}
impl PartialEq<SiteNavigationElementIriOrLabel> for &str {
	fn eq(&self, other: &SiteNavigationElementIriOrLabel) -> bool {
		*self == SiteNavigationElementIri || *self == SITE_NAVIGATION_ELEMENT_LABEL
	}
}
