/// <https://schema.org/ProductCollection>
pub const PRODUCT_COLLECTION_IRI_HTTP: &str = "http://schema.org/ProductCollection";
/// <https://schema.org/ProductCollection>
pub const PRODUCT_COLLECTION_IRI_HTTPS: &str = "https://schema.org/ProductCollection";
/// <https://schema.org/ProductCollection>
pub const PRODUCT_COLLECTION_LABEL: &str = "ProductCollection";
pub struct ProductCollectionIri;
impl PartialEq<&str> for ProductCollectionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRODUCT_COLLECTION_IRI_HTTP || *other == PRODUCT_COLLECTION_IRI_HTTPS
	}
}
impl PartialEq<ProductCollectionIri> for &str {
	fn eq(&self, other: &ProductCollectionIri) -> bool {
		*self == PRODUCT_COLLECTION_IRI_HTTP || *self == PRODUCT_COLLECTION_IRI_HTTPS
	}
}
pub struct ProductCollectionIriOrLabel;
impl PartialEq<&str> for ProductCollectionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProductCollectionIri || *other == PRODUCT_COLLECTION_LABEL
	}
}
impl PartialEq<ProductCollectionIriOrLabel> for &str {
	fn eq(&self, other: &ProductCollectionIriOrLabel) -> bool {
		*self == ProductCollectionIri || *self == PRODUCT_COLLECTION_LABEL
	}
}
