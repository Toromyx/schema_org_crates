/// <https://schema.org/WearableSizeGroupInfants>
pub const WEARABLE_SIZE_GROUP_INFANTS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupInfants";
/// <https://schema.org/WearableSizeGroupInfants>
pub const WEARABLE_SIZE_GROUP_INFANTS_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupInfants";
/// <https://schema.org/WearableSizeGroupInfants>
pub const WEARABLE_SIZE_GROUP_INFANTS_LABEL: &str = "WearableSizeGroupInfants";
pub struct WearableSizeGroupInfantsIri;
impl PartialEq<&str> for WearableSizeGroupInfantsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_INFANTS_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_INFANTS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupInfantsIri> for &str {
	fn eq(&self, other: &WearableSizeGroupInfantsIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_INFANTS_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_INFANTS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupInfantsIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupInfantsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupInfantsIri || *other == WEARABLE_SIZE_GROUP_INFANTS_LABEL
	}
}
impl PartialEq<WearableSizeGroupInfantsIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupInfantsIriOrLabel) -> bool {
		*self == WearableSizeGroupInfantsIri || *self == WEARABLE_SIZE_GROUP_INFANTS_LABEL
	}
}
