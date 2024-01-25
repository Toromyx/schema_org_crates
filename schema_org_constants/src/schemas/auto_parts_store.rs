/// <https://schema.org/AutoPartsStore>
pub const AUTO_PARTS_STORE_IRI_HTTP: &str = "http://schema.org/AutoPartsStore";
/// <https://schema.org/AutoPartsStore>
pub const AUTO_PARTS_STORE_IRI_HTTPS: &str = "https://schema.org/AutoPartsStore";
/// <https://schema.org/AutoPartsStore>
pub const AUTO_PARTS_STORE_LABEL: &str = "AutoPartsStore";
pub struct AutoPartsStoreIri;
impl PartialEq<&str> for AutoPartsStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTO_PARTS_STORE_IRI_HTTP || *other == AUTO_PARTS_STORE_IRI_HTTPS
	}
}
impl PartialEq<AutoPartsStoreIri> for &str {
	fn eq(&self, other: &AutoPartsStoreIri) -> bool {
		*self == AUTO_PARTS_STORE_IRI_HTTP || *self == AUTO_PARTS_STORE_IRI_HTTPS
	}
}
pub struct AutoPartsStoreIriOrLabel;
impl PartialEq<&str> for AutoPartsStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutoPartsStoreIri || *other == AUTO_PARTS_STORE_LABEL
	}
}
impl PartialEq<AutoPartsStoreIriOrLabel> for &str {
	fn eq(&self, other: &AutoPartsStoreIriOrLabel) -> bool {
		*self == AutoPartsStoreIri || *self == AUTO_PARTS_STORE_LABEL
	}
}
