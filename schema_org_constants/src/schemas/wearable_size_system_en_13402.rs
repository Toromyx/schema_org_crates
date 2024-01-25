/// <https://schema.org/WearableSizeSystemEN13402>
pub const WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTP: &str =
	"http://schema.org/WearableSizeSystemEN13402";
/// <https://schema.org/WearableSizeSystemEN13402>
pub const WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeSystemEN13402";
/// <https://schema.org/WearableSizeSystemEN13402>
pub const WEARABLE_SIZE_SYSTEM_EN_13402_LABEL: &str = "WearableSizeSystemEN13402";
pub struct WearableSizeSystemEn13402Iri;
impl PartialEq<&str> for WearableSizeSystemEn13402Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTP
			|| *other == WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemEn13402Iri> for &str {
	fn eq(&self, other: &WearableSizeSystemEn13402Iri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTP
			|| *self == WEARABLE_SIZE_SYSTEM_EN_13402_IRI_HTTPS
	}
}
pub struct WearableSizeSystemEn13402IriOrLabel;
impl PartialEq<&str> for WearableSizeSystemEn13402IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemEn13402Iri || *other == WEARABLE_SIZE_SYSTEM_EN_13402_LABEL
	}
}
impl PartialEq<WearableSizeSystemEn13402IriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemEn13402IriOrLabel) -> bool {
		*self == WearableSizeSystemEn13402Iri || *self == WEARABLE_SIZE_SYSTEM_EN_13402_LABEL
	}
}
