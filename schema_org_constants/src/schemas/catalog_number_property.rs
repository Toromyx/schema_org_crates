/// <https://schema.org/catalogNumber>
pub const CATALOG_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/catalogNumber";
/// <https://schema.org/catalogNumber>
pub const CATALOG_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/catalogNumber";
/// <https://schema.org/catalogNumber>
pub const CATALOG_NUMBER_PROPERTY_LABEL: &str = "catalogNumber";
pub struct CatalogNumberPropertyIri;
impl PartialEq<&str> for CatalogNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CATALOG_NUMBER_PROPERTY_IRI_HTTP || *other == CATALOG_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CatalogNumberPropertyIri> for &str {
	fn eq(&self, other: &CatalogNumberPropertyIri) -> bool {
		*self == CATALOG_NUMBER_PROPERTY_IRI_HTTP || *self == CATALOG_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct CatalogNumberPropertyIriOrLabel;
impl PartialEq<&str> for CatalogNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CatalogNumberPropertyIri || *other == CATALOG_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<CatalogNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CatalogNumberPropertyIriOrLabel) -> bool {
		*self == CatalogNumberPropertyIri || *self == CATALOG_NUMBER_PROPERTY_LABEL
	}
}
