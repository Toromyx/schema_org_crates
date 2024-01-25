/// <https://schema.org/mainEntity>
pub const MAIN_ENTITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/mainEntity";
/// <https://schema.org/mainEntity>
pub const MAIN_ENTITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mainEntity";
/// <https://schema.org/mainEntity>
pub const MAIN_ENTITY_PROPERTY_LABEL: &str = "mainEntity";
pub struct MainEntityPropertyIri;
impl PartialEq<&str> for MainEntityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAIN_ENTITY_PROPERTY_IRI_HTTP || *other == MAIN_ENTITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MainEntityPropertyIri> for &str {
	fn eq(&self, other: &MainEntityPropertyIri) -> bool {
		*self == MAIN_ENTITY_PROPERTY_IRI_HTTP || *self == MAIN_ENTITY_PROPERTY_IRI_HTTPS
	}
}
pub struct MainEntityPropertyIriOrLabel;
impl PartialEq<&str> for MainEntityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MainEntityPropertyIri || *other == MAIN_ENTITY_PROPERTY_LABEL
	}
}
impl PartialEq<MainEntityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MainEntityPropertyIriOrLabel) -> bool {
		*self == MainEntityPropertyIri || *self == MAIN_ENTITY_PROPERTY_LABEL
	}
}
