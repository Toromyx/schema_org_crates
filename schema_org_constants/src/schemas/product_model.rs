/// <https://schema.org/ProductModel>
pub const PRODUCT_MODEL_IRI_HTTP: &str = "http://schema.org/ProductModel";
/// <https://schema.org/ProductModel>
pub const PRODUCT_MODEL_IRI_HTTPS: &str = "https://schema.org/ProductModel";
/// <https://schema.org/ProductModel>
pub const PRODUCT_MODEL_LABEL: &str = "ProductModel";
pub struct ProductModelIri;
impl PartialEq<&str> for ProductModelIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_MODEL_IRI_HTTP || *other == PRODUCT_MODEL_IRI_HTTPS
	}
}
impl PartialEq<ProductModelIri> for &str {
	fn eq(&self, other: &ProductModelIri) -> bool {
		*self == PRODUCT_MODEL_IRI_HTTP || *self == PRODUCT_MODEL_IRI_HTTPS
	}
}
pub struct ProductModelIriOrLabel;
impl PartialEq<&str> for ProductModelIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductModelIri || *other == PRODUCT_MODEL_LABEL
	}
}
impl PartialEq<ProductModelIriOrLabel> for &str {
	fn eq(&self, other: &ProductModelIriOrLabel) -> bool {
		*self == ProductModelIri || *self == PRODUCT_MODEL_LABEL
	}
}
