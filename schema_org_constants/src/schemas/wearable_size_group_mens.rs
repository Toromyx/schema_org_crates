/// <https://schema.org/WearableSizeGroupMens>
pub const WEARABLE_SIZE_GROUP_MENS_IRI_HTTP: &str = "http://schema.org/WearableSizeGroupMens";
/// <https://schema.org/WearableSizeGroupMens>
pub const WEARABLE_SIZE_GROUP_MENS_IRI_HTTPS: &str = "https://schema.org/WearableSizeGroupMens";
/// <https://schema.org/WearableSizeGroupMens>
pub const WEARABLE_SIZE_GROUP_MENS_LABEL: &str = "WearableSizeGroupMens";
pub struct WearableSizeGroupMensIri;
impl PartialEq<&str> for WearableSizeGroupMensIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_GROUP_MENS_IRI_HTTP || *other == WEARABLE_SIZE_GROUP_MENS_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeGroupMensIri> for &str {
	fn eq(&self, other: &WearableSizeGroupMensIri) -> bool {
		*self == WEARABLE_SIZE_GROUP_MENS_IRI_HTTP || *self == WEARABLE_SIZE_GROUP_MENS_IRI_HTTPS
	}
}
pub struct WearableSizeGroupMensIriOrLabel;
impl PartialEq<&str> for WearableSizeGroupMensIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeGroupMensIri || *other == WEARABLE_SIZE_GROUP_MENS_LABEL
	}
}
impl PartialEq<WearableSizeGroupMensIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeGroupMensIriOrLabel) -> bool {
		*self == WearableSizeGroupMensIri || *self == WEARABLE_SIZE_GROUP_MENS_LABEL
	}
}
