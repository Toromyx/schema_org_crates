/// <https://schema.org/FurnitureStore>
pub const FURNITURE_STORE_IRI_HTTP: &str = "http://schema.org/FurnitureStore";
/// <https://schema.org/FurnitureStore>
pub const FURNITURE_STORE_IRI_HTTPS: &str = "https://schema.org/FurnitureStore";
/// <https://schema.org/FurnitureStore>
pub const FURNITURE_STORE_LABEL: &str = "FurnitureStore";
pub struct FurnitureStoreIri;
impl PartialEq<&str> for FurnitureStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FURNITURE_STORE_IRI_HTTP || *other == FURNITURE_STORE_IRI_HTTPS
	}
}
impl PartialEq<FurnitureStoreIri> for &str {
	fn eq(&self, other: &FurnitureStoreIri) -> bool {
		*self == FURNITURE_STORE_IRI_HTTP || *self == FURNITURE_STORE_IRI_HTTPS
	}
}
pub struct FurnitureStoreIriOrLabel;
impl PartialEq<&str> for FurnitureStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FurnitureStoreIri || *other == FURNITURE_STORE_LABEL
	}
}
impl PartialEq<FurnitureStoreIriOrLabel> for &str {
	fn eq(&self, other: &FurnitureStoreIriOrLabel) -> bool {
		*self == FurnitureStoreIri || *self == FURNITURE_STORE_LABEL
	}
}
