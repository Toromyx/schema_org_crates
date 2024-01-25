/// <https://schema.org/WearableSizeGroupJuniors>
pub const WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupJuniors";
/// <https://schema.org/WearableSizeGroupJuniors>
pub const WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeGroupJuniors";
/// <https://schema.org/WearableSizeGroupJuniors>
pub const WEARABLE_SIZE_GROUP_JUNIORS_LABEL: &str = "WearableSizeGroupJuniors";
pub struct WearableSizeGroupJuniorsIri;
impl PartialEq<&str> for WearableSizeGroupJuniorsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTP
			|| *other == WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupJuniorsIri> for &str {
	fn eq(&self, other: &WearableSizeGroupJuniorsIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTP
			|| *self == WEARABLE_SIZE_GROUP_JUNIORS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupJuniorsIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupJuniorsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupJuniorsIri || *other == WEARABLE_SIZE_GROUP_JUNIORS_LABEL
	}
}
impl PartialEq<WearableSizeGroupJuniorsIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupJuniorsIriOrLabel) -> bool {
		*self == WearableSizeGroupJuniorsIri || *self == WEARABLE_SIZE_GROUP_JUNIORS_LABEL
	}
}
