/// <https://schema.org/productSupported>
pub const PRODUCT_SUPPORTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/productSupported";
/// <https://schema.org/productSupported>
pub const PRODUCT_SUPPORTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/productSupported";
/// <https://schema.org/productSupported>
pub const PRODUCT_SUPPORTED_PROPERTY_LABEL: &str = "productSupported";
pub struct ProductSupportedPropertyIri;
impl PartialEq<&str> for ProductSupportedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_SUPPORTED_PROPERTY_IRI_HTTP
			|| *other == PRODUCT_SUPPORTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProductSupportedPropertyIri> for &str {
	fn eq(&self, other: &ProductSupportedPropertyIri) -> bool {
		*self == PRODUCT_SUPPORTED_PROPERTY_IRI_HTTP
			|| *self == PRODUCT_SUPPORTED_PROPERTY_IRI_HTTPS
	}
}
pub struct ProductSupportedPropertyIriOrLabel;
impl PartialEq<&str> for ProductSupportedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductSupportedPropertyIri || *other == PRODUCT_SUPPORTED_PROPERTY_LABEL
	}
}
impl PartialEq<ProductSupportedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProductSupportedPropertyIriOrLabel) -> bool {
		*self == ProductSupportedPropertyIri || *self == PRODUCT_SUPPORTED_PROPERTY_LABEL
	}
}
