/// <https://schema.org/productID>
pub const PRODUCT_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/productID";
/// <https://schema.org/productID>
pub const PRODUCT_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productID";
/// <https://schema.org/productID>
pub const PRODUCT_ID_PROPERTY_LABEL: &str = "productID";
pub struct ProductIdPropertyIri;
impl PartialEq<&str> for ProductIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_ID_PROPERTY_IRI_HTTP || *other == PRODUCT_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductIdPropertyIri> for &str {
	fn eq(&self, other: &ProductIdPropertyIri) -> bool {
		*self == PRODUCT_ID_PROPERTY_IRI_HTTP || *self == PRODUCT_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductIdPropertyIriOrLabel;
impl PartialEq<&str> for ProductIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductIdPropertyIri || *other == PRODUCT_ID_PROPERTY_LABEL
	}
}
impl PartialEq<ProductIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductIdPropertyIriOrLabel) -> bool {
		*self == ProductIdPropertyIri || *self == PRODUCT_ID_PROPERTY_LABEL
	}
}
