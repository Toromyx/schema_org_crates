/// <https://schema.org/DataCatalog>
pub const DATA_CATALOG_IRI_HTTP: &str = "http://schema.org/DataCatalog";
/// <https://schema.org/DataCatalog>
pub const DATA_CATALOG_IRI_HTTPS: &str = "https://schema.org/DataCatalog";
/// <https://schema.org/DataCatalog>
pub const DATA_CATALOG_LABEL: &str = "DataCatalog";
pub struct DataCatalogIri;
impl PartialEq<&str> for DataCatalogIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATA_CATALOG_IRI_HTTP || *other == DATA_CATALOG_IRI_HTTPS
	}
}
impl PartialEq<DataCatalogIri> for &str {
	fn eq(&self, other: &DataCatalogIri) -> bool {
		*self == DATA_CATALOG_IRI_HTTP || *self == DATA_CATALOG_IRI_HTTPS
	}
}
pub struct DataCatalogIriOrLabel;
impl PartialEq<&str> for DataCatalogIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DataCatalogIri || *other == DATA_CATALOG_LABEL
	}
}
impl PartialEq<DataCatalogIriOrLabel> for &str {
	fn eq(&self, other: &DataCatalogIriOrLabel) -> bool {
		*self == DataCatalogIri || *self == DATA_CATALOG_LABEL
	}
}
