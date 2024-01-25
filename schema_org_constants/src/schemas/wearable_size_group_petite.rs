/// <https://schema.org/WearableSizeGroupPetite>
pub const WEARABLE_SIZE_GROUP_PETITE_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupPetite";
/// <https://schema.org/WearableSizeGroupPetite>
pub const WEARABLE_SIZE_GROUP_PETITE_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupPetite";
/// <https://schema.org/WearableSizeGroupPetite>
pub const WEARABLE_SIZE_GROUP_PETITE_LABEL: &str = "WearableSizeGroupPetite";
pub struct WearableSizeGroupPetiteIri;
impl PartialEq<&str> for WearableSizeGroupPetiteIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_PETITE_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_PETITE_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupPetiteIri> for &str {
	fn eq(&self, other: &WearableSizeGroupPetiteIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_PETITE_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_PETITE_IRI_HTTPS
	}
}
pub struct WearableSizeGroupPetiteIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupPetiteIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupPetiteIri || *other == WEARABLE_SIZE_GROUP_PETITE_LABEL
	}
}
impl PartialEq<WearableSizeGroupPetiteIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupPetiteIriOrLabel) -> bool {
		*self == WearableSizeGroupPetiteIri || *self == WEARABLE_SIZE_GROUP_PETITE_LABEL
	}
}
