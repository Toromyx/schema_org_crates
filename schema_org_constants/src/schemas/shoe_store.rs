/// <https://schema.org/ShoeStore>
pub const SHOE_STORE_IRI_HTTP: &str = "http://schema.org/ShoeStore";
/// <https://schema.org/ShoeStore>
pub const SHOE_STORE_IRI_HTTPS: &str = "https://schema.org/ShoeStore";
/// <https://schema.org/ShoeStore>
pub const SHOE_STORE_LABEL: &str = "ShoeStore";
pub struct ShoeStoreIri;
impl PartialEq<&str> for ShoeStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHOE_STORE_IRI_HTTP || *other == SHOE_STORE_IRI_HTTPS
	}
}
impl PartialEq<ShoeStoreIri> for &str {
	fn eq(&self, other: &ShoeStoreIri) -> bool {
		*self == SHOE_STORE_IRI_HTTP || *self == SHOE_STORE_IRI_HTTPS
	}
}
pub struct ShoeStoreIriOrLabel;
impl PartialEq<&str> for ShoeStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShoeStoreIri || *other == SHOE_STORE_LABEL
	}
}
impl PartialEq<ShoeStoreIriOrLabel> for &str {
	fn eq(&self, other: &ShoeStoreIriOrLabel) -> bool {
		*self == ShoeStoreIri || *self == SHOE_STORE_LABEL
	}
}
