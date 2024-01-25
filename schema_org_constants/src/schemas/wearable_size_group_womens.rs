/// <https://schema.org/WearableSizeGroupWomens>
pub const WEARABLE_SIZE_GROUP_WOMENS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupWomens";
/// <https://schema.org/WearableSizeGroupWomens>
pub const WEARABLE_SIZE_GROUP_WOMENS_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupWomens";
/// <https://schema.org/WearableSizeGroupWomens>
pub const WEARABLE_SIZE_GROUP_WOMENS_LABEL: &str = "WearableSizeGroupWomens";
pub struct WearableSizeGroupWomensIri;
impl PartialEq<&str> for WearableSizeGroupWomensIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_WOMENS_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_WOMENS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupWomensIri> for &str {
	fn eq(&self, other: &WearableSizeGroupWomensIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_WOMENS_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_WOMENS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupWomensIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupWomensIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupWomensIri || *other == WEARABLE_SIZE_GROUP_WOMENS_LABEL
	}
}
impl PartialEq<WearableSizeGroupWomensIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupWomensIriOrLabel) -> bool {
		*self == WearableSizeGroupWomensIri || *self == WEARABLE_SIZE_GROUP_WOMENS_LABEL
	}
}
