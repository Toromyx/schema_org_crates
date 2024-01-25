/// <https://schema.org/JewelryStore>
pub const JEWELRY_STORE_IRI_HTTP: &str = "http://schema.org/JewelryStore";
/// <https://schema.org/JewelryStore>
pub const JEWELRY_STORE_IRI_HTTPS: &str = "https://schema.org/JewelryStore";
/// <https://schema.org/JewelryStore>
pub const JEWELRY_STORE_LABEL: &str = "JewelryStore";
pub struct JewelryStoreIri;
impl PartialEq<&str> for JewelryStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == JEWELRY_STORE_IRI_HTTP || *other == JEWELRY_STORE_IRI_HTTPS
	}
}
impl PartialEq<JewelryStoreIri> for &str {
	fn eq(&self, other: &JewelryStoreIri) -> bool {
		*self == JEWELRY_STORE_IRI_HTTP || *self == JEWELRY_STORE_IRI_HTTPS
	}
}
pub struct JewelryStoreIriOrLabel;
impl PartialEq<&str> for JewelryStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == JewelryStoreIri || *other == JEWELRY_STORE_LABEL
	}
}
impl PartialEq<JewelryStoreIriOrLabel> for &str {
	fn eq(&self, other: &JewelryStoreIriOrLabel) -> bool {
		*self == JewelryStoreIri || *self == JEWELRY_STORE_LABEL
	}
}
