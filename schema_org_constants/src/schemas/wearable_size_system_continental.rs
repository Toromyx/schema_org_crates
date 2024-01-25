/// <https://schema.org/WearableSizeSystemContinental>
pub const WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTP: &str =
	"http://schema.org/WearableSizeSystemContinental";
/// <https://schema.org/WearableSizeSystemContinental>
pub const WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeSystemContinental";
/// <https://schema.org/WearableSizeSystemContinental>
pub const WEARABLE_SIZE_SYSTEM_CONTINENTAL_LABEL: &str = "WearableSizeSystemContinental";
pub struct WearableSizeSystemContinentalIri;
impl PartialEq<&str> for WearableSizeSystemContinentalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTP
			|| *other == WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemContinentalIri> for &str {
	fn eq(&self, other: &WearableSizeSystemContinentalIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTP
			|| *self == WEARABLE_SIZE_SYSTEM_CONTINENTAL_IRI_HTTPS
	}
}
pub struct WearableSizeSystemContinentalIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemContinentalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemContinentalIri
			|| *other == WEARABLE_SIZE_SYSTEM_CONTINENTAL_LABEL
	}
}
impl PartialEq<WearableSizeSystemContinentalIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemContinentalIriOrLabel) -> bool {
		*self == WearableSizeSystemContinentalIri || *self == WEARABLE_SIZE_SYSTEM_CONTINENTAL_LABEL
	}
}
