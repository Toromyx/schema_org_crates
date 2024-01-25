/// <https://schema.org/OfferCatalog>
pub const OFFER_CATALOG_IRI_HTTP: &str = "http://schema.org/OfferCatalog";
/// <https://schema.org/OfferCatalog>
pub const OFFER_CATALOG_IRI_HTTPS: &str = "https://schema.org/OfferCatalog";
/// <https://schema.org/OfferCatalog>
pub const OFFER_CATALOG_LABEL: &str = "OfferCatalog";
pub struct OfferCatalogIri;
impl PartialEq<&str> for OfferCatalogIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFER_CATALOG_IRI_HTTP || *other == OFFER_CATALOG_IRI_HTTPS
	}
}
impl PartialEq<OfferCatalogIri> for &str {
	fn eq(&self, other: &OfferCatalogIri) -> bool {
		*self == OFFER_CATALOG_IRI_HTTP || *self == OFFER_CATALOG_IRI_HTTPS
	}
}
pub struct OfferCatalogIriOrLabel;
impl PartialEq<&str> for OfferCatalogIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfferCatalogIri || *other == OFFER_CATALOG_LABEL
	}
}
impl PartialEq<OfferCatalogIriOrLabel> for &str {
	fn eq(&self, other: &OfferCatalogIriOrLabel) -> bool {
		*self == OfferCatalogIri || *self == OFFER_CATALOG_LABEL
	}
}
