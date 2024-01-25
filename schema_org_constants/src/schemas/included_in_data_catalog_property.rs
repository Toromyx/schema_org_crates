/// <https://schema.org/includedInDataCatalog>
pub const INCLUDED_IN_DATA_CATALOG_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/includedInDataCatalog";
/// <https://schema.org/includedInDataCatalog>
pub const INCLUDED_IN_DATA_CATALOG_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/includedInDataCatalog";
/// <https://schema.org/includedInDataCatalog>
pub const INCLUDED_IN_DATA_CATALOG_PROPERTY_LABEL: &str = "includedInDataCatalog";
pub struct IncludedInDataCatalogPropertyIri;
impl PartialEq<&str> for IncludedInDataCatalogPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDED_IN_DATA_CATALOG_PROPERTY_IRI_HTTP
			|| *other == INCLUDED_IN_DATA_CATALOG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludedInDataCatalogPropertyIri> for &str {
	fn eq(&self, other: &IncludedInDataCatalogPropertyIri) -> bool {
		*self == INCLUDED_IN_DATA_CATALOG_PROPERTY_IRI_HTTP
			|| *self == INCLUDED_IN_DATA_CATALOG_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludedInDataCatalogPropertyIriOrLabel;
impl PartialEq<&str> for IncludedInDataCatalogPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludedInDataCatalogPropertyIri
			|| *other == INCLUDED_IN_DATA_CATALOG_PROPERTY_LABEL
	}
}
impl PartialEq<IncludedInDataCatalogPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludedInDataCatalogPropertyIriOrLabel) -> bool {
		*self == IncludedInDataCatalogPropertyIri
			|| *self == INCLUDED_IN_DATA_CATALOG_PROPERTY_LABEL
	}
}
