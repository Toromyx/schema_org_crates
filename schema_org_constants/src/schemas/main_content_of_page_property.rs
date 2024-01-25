/// <https://schema.org/mainContentOfPage>
pub const MAIN_CONTENT_OF_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/mainContentOfPage";
/// <https://schema.org/mainContentOfPage>
pub const MAIN_CONTENT_OF_PAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mainContentOfPage";
/// <https://schema.org/mainContentOfPage>
pub const MAIN_CONTENT_OF_PAGE_PROPERTY_LABEL: &str = "mainContentOfPage";
pub struct MainContentOfPagePropertyIri;
impl PartialEq<&str> for MainContentOfPagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAIN_CONTENT_OF_PAGE_PROPERTY_IRI_HTTP
			|| *other == MAIN_CONTENT_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MainContentOfPagePropertyIri> for &str {
	fn eq(&self, other: &MainContentOfPagePropertyIri) -> bool {
		*self == MAIN_CONTENT_OF_PAGE_PROPERTY_IRI_HTTP
			|| *self == MAIN_CONTENT_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct MainContentOfPagePropertyIriOrLabel;
impl PartialEq<&str> for MainContentOfPagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MainContentOfPagePropertyIri || *other == MAIN_CONTENT_OF_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<MainContentOfPagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MainContentOfPagePropertyIriOrLabel) -> bool {
		*self == MainContentOfPagePropertyIri || *self == MAIN_CONTENT_OF_PAGE_PROPERTY_LABEL
	}
}
