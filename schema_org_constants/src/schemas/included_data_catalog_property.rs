/// <https://schema.org/includedDataCatalog>
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub const INCLUDED_DATA_CATALOG_PROPERTY_IRI_HTTP: &str = "http://schema.org/includedDataCatalog";
/// <https://schema.org/includedDataCatalog>
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub const INCLUDED_DATA_CATALOG_PROPERTY_IRI_HTTPS: &str = "https://schema.org/includedDataCatalog";
/// <https://schema.org/includedDataCatalog>
#[deprecated = "This schema is superseded by <https://schema.org/includedInDataCatalog>."]
pub const INCLUDED_DATA_CATALOG_PROPERTY_LABEL: &str = "includedDataCatalog";
pub struct IncludedDataCatalogPropertyIri;
impl PartialEq<&str> for IncludedDataCatalogPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDED_DATA_CATALOG_PROPERTY_IRI_HTTP
			|| *other == INCLUDED_DATA_CATALOG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludedDataCatalogPropertyIri> for &str {
	fn eq(&self, other: &IncludedDataCatalogPropertyIri) -> bool {
		*self == INCLUDED_DATA_CATALOG_PROPERTY_IRI_HTTP
			|| *self == INCLUDED_DATA_CATALOG_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludedDataCatalogPropertyIriOrLabel;
impl PartialEq<&str> for IncludedDataCatalogPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludedDataCatalogPropertyIri || *other == INCLUDED_DATA_CATALOG_PROPERTY_LABEL
	}
}
impl PartialEq<IncludedDataCatalogPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludedDataCatalogPropertyIriOrLabel) -> bool {
		*self == IncludedDataCatalogPropertyIri || *self == INCLUDED_DATA_CATALOG_PROPERTY_LABEL
	}
}
