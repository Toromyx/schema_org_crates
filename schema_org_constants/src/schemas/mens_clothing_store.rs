/// <https://schema.org/MensClothingStore>
pub const MENS_CLOTHING_STORE_IRI_HTTP: &str = "http://schema.org/MensClothingStore";
/// <https://schema.org/MensClothingStore>
pub const MENS_CLOTHING_STORE_IRI_HTTPS: &str = "https://schema.org/MensClothingStore";
/// <https://schema.org/MensClothingStore>
pub const MENS_CLOTHING_STORE_LABEL: &str = "MensClothingStore";
pub struct MensClothingStoreIri;
impl PartialEq<&str> for MensClothingStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MENS_CLOTHING_STORE_IRI_HTTP || *other == MENS_CLOTHING_STORE_IRI_HTTPS
	}
}
impl PartialEq<MensClothingStoreIri> for &str {
	fn eq(&self, other: &MensClothingStoreIri) -> bool {
		*self == MENS_CLOTHING_STORE_IRI_HTTP || *self == MENS_CLOTHING_STORE_IRI_HTTPS
	}
}
pub struct MensClothingStoreIriOrLabel;
impl PartialEq<&str> for MensClothingStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MensClothingStoreIri || *other == MENS_CLOTHING_STORE_LABEL
	}
}
impl PartialEq<MensClothingStoreIriOrLabel> for &str {
	fn eq(&self, other: &MensClothingStoreIriOrLabel) -> bool {
		*self == MensClothingStoreIri || *self == MENS_CLOTHING_STORE_LABEL
	}
}
