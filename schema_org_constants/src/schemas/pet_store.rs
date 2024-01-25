/// <https://schema.org/PetStore>
pub const PET_STORE_IRI_HTTP: &str = "http://schema.org/PetStore";
/// <https://schema.org/PetStore>
pub const PET_STORE_IRI_HTTPS: &str = "https://schema.org/PetStore";
/// <https://schema.org/PetStore>
pub const PET_STORE_LABEL: &str = "PetStore";
pub struct PetStoreIri;
impl PartialEq<&str> for PetStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PET_STORE_IRI_HTTP || *other == PET_STORE_IRI_HTTPS
	}
}
impl PartialEq<PetStoreIri> for &str {
	fn eq(&self, other: &PetStoreIri) -> bool {
		*self == PET_STORE_IRI_HTTP || *self == PET_STORE_IRI_HTTPS
	}
}
pub struct PetStoreIriOrLabel;
impl PartialEq<&str> for PetStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PetStoreIri || *other == PET_STORE_LABEL
	}
}
impl PartialEq<PetStoreIriOrLabel> for &str {
	fn eq(&self, other: &PetStoreIriOrLabel) -> bool {
		*self == PetStoreIri || *self == PET_STORE_LABEL
	}
}
