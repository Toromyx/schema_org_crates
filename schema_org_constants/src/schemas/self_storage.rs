/// <https://schema.org/SelfStorage>
pub const SELF_STORAGE_IRI_HTTP: &str = "http://schema.org/SelfStorage";
/// <https://schema.org/SelfStorage>
pub const SELF_STORAGE_IRI_HTTPS: &str = "https://schema.org/SelfStorage";
/// <https://schema.org/SelfStorage>
pub const SELF_STORAGE_LABEL: &str = "SelfStorage";
pub struct SelfStorageIri;
impl PartialEq<&str> for SelfStorageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SELF_STORAGE_IRI_HTTP || *other == SELF_STORAGE_IRI_HTTPS
	}
}
impl PartialEq<SelfStorageIri> for &str {
	fn eq(&self, other: &SelfStorageIri) -> bool {
		*self == SELF_STORAGE_IRI_HTTP || *self == SELF_STORAGE_IRI_HTTPS
	}
}
pub struct SelfStorageIriOrLabel;
impl PartialEq<&str> for SelfStorageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SelfStorageIri || *other == SELF_STORAGE_LABEL
	}
}
impl PartialEq<SelfStorageIriOrLabel> for &str {
	fn eq(&self, other: &SelfStorageIriOrLabel) -> bool {
		*self == SelfStorageIri || *self == SELF_STORAGE_LABEL
	}
}
