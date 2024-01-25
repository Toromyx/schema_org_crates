/// <https://schema.org/ElectronicsStore>
pub const ELECTRONICS_STORE_IRI_HTTP: &str = "http://schema.org/ElectronicsStore";
/// <https://schema.org/ElectronicsStore>
pub const ELECTRONICS_STORE_IRI_HTTPS: &str = "https://schema.org/ElectronicsStore";
/// <https://schema.org/ElectronicsStore>
pub const ELECTRONICS_STORE_LABEL: &str = "ElectronicsStore";
pub struct ElectronicsStoreIri;
impl PartialEq<&str> for ElectronicsStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ELECTRONICS_STORE_IRI_HTTP || *other == ELECTRONICS_STORE_IRI_HTTPS
	}
}
impl PartialEq<ElectronicsStoreIri> for &str {
	fn eq(&self, other: &ElectronicsStoreIri) -> bool {
		*self == ELECTRONICS_STORE_IRI_HTTP || *self == ELECTRONICS_STORE_IRI_HTTPS
	}
}
pub struct ElectronicsStoreIriOrLabel;
impl PartialEq<&str> for ElectronicsStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ElectronicsStoreIri || *other == ELECTRONICS_STORE_LABEL
	}
}
impl PartialEq<ElectronicsStoreIriOrLabel> for &str {
	fn eq(&self, other: &ElectronicsStoreIriOrLabel) -> bool {
		*self == ElectronicsStoreIri || *self == ELECTRONICS_STORE_LABEL
	}
}
