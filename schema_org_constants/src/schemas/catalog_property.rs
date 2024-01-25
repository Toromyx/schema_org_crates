/// <https://schema.org/catalog>
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub const CATALOG_PROPERTY_IRI_HTTP: &str = "http://schema.org/catalog";
/// <https://schema.org/catalog>
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub const CATALOG_PROPERTY_IRI_HTTPS: &str = "https://schema.org/catalog";
/// <https://schema.org/catalog>
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub const CATALOG_PROPERTY_LABEL: &str = "catalog";
pub struct CatalogPropertyIri;
impl PartialEq<&str> for CatalogPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CATALOG_PROPERTY_IRI_HTTP || *other == CATALOG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CatalogPropertyIri> for &str {
	fn eq(&self, other: &CatalogPropertyIri) -> bool {
		*self == CATALOG_PROPERTY_IRI_HTTP || *self == CATALOG_PROPERTY_IRI_HTTPS
	}
}
pub struct CatalogPropertyIriOrLabel;
impl PartialEq<&str> for CatalogPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CatalogPropertyIri || *other == CATALOG_PROPERTY_LABEL
	}
}
impl PartialEq<CatalogPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CatalogPropertyIriOrLabel) -> bool {
		*self == CatalogPropertyIri || *self == CATALOG_PROPERTY_LABEL
	}
}
