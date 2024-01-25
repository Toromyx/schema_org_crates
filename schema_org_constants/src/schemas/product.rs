/// <https://schema.org/Product>
pub const PRODUCT_IRI_HTTP: &str = "http://schema.org/Product";
/// <https://schema.org/Product>
pub const PRODUCT_IRI_HTTPS: &str = "https://schema.org/Product";
/// <https://schema.org/Product>
pub const PRODUCT_LABEL: &str = "Product";
pub struct ProductIri;
impl PartialEq<&str> for ProductIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_IRI_HTTP || *other == PRODUCT_IRI_HTTPS
	}
}
impl PartialEq<ProductIri> for &str {
	fn eq(&self, other: &ProductIri) -> bool {
		*self == PRODUCT_IRI_HTTP || *self == PRODUCT_IRI_HTTPS
	}
}
pub struct ProductIriOrLabel;
impl PartialEq<&str> for ProductIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductIri || *other == PRODUCT_LABEL
	}
}
impl PartialEq<ProductIriOrLabel> for &str {
	fn eq(&self, other: &ProductIriOrLabel) -> bool {
		*self == ProductIri || *self == PRODUCT_LABEL
	}
}
