/// <https://schema.org/KeepProduct>
pub const KEEP_PRODUCT_IRI_HTTP: &str = "http://schema.org/KeepProduct";
/// <https://schema.org/KeepProduct>
pub const KEEP_PRODUCT_IRI_HTTPS: &str = "https://schema.org/KeepProduct";
/// <https://schema.org/KeepProduct>
pub const KEEP_PRODUCT_LABEL: &str = "KeepProduct";
pub struct KeepProductIri;
impl PartialEq<&str> for KeepProductIri {
	fn eq(&self, other: &&str) -> bool {
		*other == KEEP_PRODUCT_IRI_HTTP || *other == KEEP_PRODUCT_IRI_HTTPS
	}
}
impl PartialEq<KeepProductIri> for &str {
	fn eq(&self, other: &KeepProductIri) -> bool {
		*self == KEEP_PRODUCT_IRI_HTTP || *self == KEEP_PRODUCT_IRI_HTTPS
	}
}
pub struct KeepProductIriOrLabel;
impl PartialEq<&str> for KeepProductIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == KeepProductIri || *other == KEEP_PRODUCT_LABEL
	}
}
impl PartialEq<KeepProductIriOrLabel> for &str {
	fn eq(&self, other: &KeepProductIriOrLabel) -> bool {
		*self == KeepProductIri || *self == KEEP_PRODUCT_LABEL
	}
}
