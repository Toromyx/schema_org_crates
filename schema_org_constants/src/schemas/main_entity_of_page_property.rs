/// <https://schema.org/mainEntityOfPage>
pub const MAIN_ENTITY_OF_PAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/mainEntityOfPage";
/// <https://schema.org/mainEntityOfPage>
pub const MAIN_ENTITY_OF_PAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mainEntityOfPage";
/// <https://schema.org/mainEntityOfPage>
pub const MAIN_ENTITY_OF_PAGE_PROPERTY_LABEL: &str = "mainEntityOfPage";
pub struct MainEntityOfPagePropertyIri;
impl PartialEq<&str> for MainEntityOfPagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAIN_ENTITY_OF_PAGE_PROPERTY_IRI_HTTP
			|| *other == MAIN_ENTITY_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MainEntityOfPagePropertyIri> for &str {
	fn eq(&self, other: &MainEntityOfPagePropertyIri) -> bool {
		*self == MAIN_ENTITY_OF_PAGE_PROPERTY_IRI_HTTP
			|| *self == MAIN_ENTITY_OF_PAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct MainEntityOfPagePropertyIriOrLabel;
impl PartialEq<&str> for MainEntityOfPagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MainEntityOfPagePropertyIri || *other == MAIN_ENTITY_OF_PAGE_PROPERTY_LABEL
	}
}
impl PartialEq<MainEntityOfPagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MainEntityOfPagePropertyIriOrLabel) -> bool {
		*self == MainEntityOfPagePropertyIri || *self == MAIN_ENTITY_OF_PAGE_PROPERTY_LABEL
	}
}
