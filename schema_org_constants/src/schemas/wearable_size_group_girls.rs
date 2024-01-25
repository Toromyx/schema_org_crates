/// <https://schema.org/WearableSizeGroupGirls>
pub const WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupGirls";
/// <https://schema.org/WearableSizeGroupGirls>
pub const WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupGirls";
/// <https://schema.org/WearableSizeGroupGirls>
pub const WEARABLE_SIZE_GROUP_GIRLS_LABEL: &str = "WearableSizeGroupGirls";
pub struct WearableSizeGroupGirlsIri;
impl PartialEq<&str> for WearableSizeGroupGirlsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupGirlsIri> for &str {
	fn eq(&self, other: &WearableSizeGroupGirlsIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_GIRLS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupGirlsIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupGirlsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupGirlsIri || *other == WEARABLE_SIZE_GROUP_GIRLS_LABEL
	}
}
impl PartialEq<WearableSizeGroupGirlsIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupGirlsIriOrLabel) -> bool {
		*self == WearableSizeGroupGirlsIri || *self == WEARABLE_SIZE_GROUP_GIRLS_LABEL
	}
}
