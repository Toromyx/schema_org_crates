/// <https://schema.org/ProductGroup>
pub const PRODUCT_GROUP_IRI_HTTP: &str = "http://schema.org/ProductGroup";
/// <https://schema.org/ProductGroup>
pub const PRODUCT_GROUP_IRI_HTTPS: &str = "https://schema.org/ProductGroup";
/// <https://schema.org/ProductGroup>
pub const PRODUCT_GROUP_LABEL: &str = "ProductGroup";
pub struct ProductGroupIri;
impl PartialEq<&str> for ProductGroupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_GROUP_IRI_HTTP || *other == PRODUCT_GROUP_IRI_HTTPS
	}
}
impl PartialEq<ProductGroupIri> for &str {
	fn eq(&self, other: &ProductGroupIri) -> bool {
		*self == PRODUCT_GROUP_IRI_HTTP || *self == PRODUCT_GROUP_IRI_HTTPS
	}
}
pub struct ProductGroupIriOrLabel;
impl PartialEq<&str> for ProductGroupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductGroupIri || *other == PRODUCT_GROUP_LABEL
	}
}
impl PartialEq<ProductGroupIriOrLabel> for &str {
	fn eq(&self, other: &ProductGroupIriOrLabel) -> bool {
		*self == ProductGroupIri || *self == PRODUCT_GROUP_LABEL
	}
}
