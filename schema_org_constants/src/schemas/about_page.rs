/// <https://schema.org/AboutPage>
pub const ABOUT_PAGE_IRI_HTTP: &str = "http://schema.org/AboutPage";
/// <https://schema.org/AboutPage>
pub const ABOUT_PAGE_IRI_HTTPS: &str = "https://schema.org/AboutPage";
/// <https://schema.org/AboutPage>
pub const ABOUT_PAGE_LABEL: &str = "AboutPage";
pub struct AboutPageIri;
impl PartialEq<&str> for AboutPageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ABOUT_PAGE_IRI_HTTP || *other == ABOUT_PAGE_IRI_HTTPS
	}
}
impl PartialEq<AboutPageIri> for &str {
	fn eq(&self, other: &AboutPageIri) -> bool {
		*self == ABOUT_PAGE_IRI_HTTP || *self == ABOUT_PAGE_IRI_HTTPS
	}
}
pub struct AboutPageIriOrLabel;
impl PartialEq<&str> for AboutPageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AboutPageIri || *other == ABOUT_PAGE_LABEL
	}
}
impl PartialEq<AboutPageIriOrLabel> for &str {
	fn eq(&self, other: &AboutPageIriOrLabel) -> bool {
		*self == AboutPageIri || *self == ABOUT_PAGE_LABEL
	}
}
