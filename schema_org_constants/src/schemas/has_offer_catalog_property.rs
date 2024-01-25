/// <https://schema.org/hasOfferCatalog>
pub const HAS_OFFER_CATALOG_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasOfferCatalog";
/// <https://schema.org/hasOfferCatalog>
pub const HAS_OFFER_CATALOG_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasOfferCatalog";
/// <https://schema.org/hasOfferCatalog>
pub const HAS_OFFER_CATALOG_PROPERTY_LABEL: &str = "hasOfferCatalog";
pub struct HasOfferCatalogPropertyIri;
impl PartialEq<&str> for HasOfferCatalogPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_OFFER_CATALOG_PROPERTY_IRI_HTTP
			|| *other == HAS_OFFER_CATALOG_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasOfferCatalogPropertyIri> for &str {
	fn eq(&self, other: &HasOfferCatalogPropertyIri) -> bool {
		*self == HAS_OFFER_CATALOG_PROPERTY_IRI_HTTP
			|| *self == HAS_OFFER_CATALOG_PROPERTY_IRI_HTTPS
	}
}
pub struct HasOfferCatalogPropertyIriOrLabel;
impl PartialEq<&str> for HasOfferCatalogPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasOfferCatalogPropertyIri || *other == HAS_OFFER_CATALOG_PROPERTY_LABEL
	}
}
impl PartialEq<HasOfferCatalogPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasOfferCatalogPropertyIriOrLabel) -> bool {
		*self == HasOfferCatalogPropertyIri || *self == HAS_OFFER_CATALOG_PROPERTY_LABEL
	}
}
