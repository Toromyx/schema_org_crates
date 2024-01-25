/// <https://schema.org/ClothingStore>
pub const CLOTHING_STORE_IRI_HTTP: &str = "http://schema.org/ClothingStore";
/// <https://schema.org/ClothingStore>
pub const CLOTHING_STORE_IRI_HTTPS: &str = "https://schema.org/ClothingStore";
/// <https://schema.org/ClothingStore>
pub const CLOTHING_STORE_LABEL: &str = "ClothingStore";
pub struct ClothingStoreIri;
impl PartialEq<&str> for ClothingStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLOTHING_STORE_IRI_HTTP || *other == CLOTHING_STORE_IRI_HTTPS
	}
}
impl PartialEq<ClothingStoreIri> for &str {
	fn eq(&self, other: &ClothingStoreIri) -> bool {
		*self == CLOTHING_STORE_IRI_HTTP || *self == CLOTHING_STORE_IRI_HTTPS
	}
}
pub struct ClothingStoreIriOrLabel;
impl PartialEq<&str> for ClothingStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClothingStoreIri || *other == CLOTHING_STORE_LABEL
	}
}
impl PartialEq<ClothingStoreIriOrLabel> for &str {
	fn eq(&self, other: &ClothingStoreIriOrLabel) -> bool {
		*self == ClothingStoreIri || *self == CLOTHING_STORE_LABEL
	}
}
