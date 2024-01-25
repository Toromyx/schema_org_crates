/// <https://schema.org/WearableSizeSystemEurope>
pub const WEARABLE_SIZE_SYSTEM_EUROPE_IRI_HTTP: &str = "http://schema.org/WearableSizeSystemEurope";
/// <https://schema.org/WearableSizeSystemEurope>
pub const WEARABLE_SIZE_SYSTEM_EUROPE_IRI_HTTPS: &str =
	"https://schema.org/WearableSizeSystemEurope";
/// <https://schema.org/WearableSizeSystemEurope>
pub const WEARABLE_SIZE_SYSTEM_EUROPE_LABEL: &str = "WearableSizeSystemEurope";
pub struct WearableSizeSystemEuropeIri;
impl PartialEq<&str> for WearableSizeSystemEuropeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_SIZE_SYSTEM_EUROPE_IRI_HTTP
			|| *other == WEARABLE_SIZE_SYSTEM_EUROPE_IRI_HTTPS
	}
}
impl PartialEq<WearableSizeSystemEuropeIri> for &str {
	fn eq(&self, other: &WearableSizeSystemEuropeIri) -> bool {
		*self == WEARABLE_SIZE_SYSTEM_EUROPE_IRI_HTTP
			|| *self == WEARABLE_SIZE_SYSTEM_EUROPE_IRI_HTTPS
	}
}
pub struct WearableSizeSystemEuropeIriOrLabel;
impl PartialEq<&str> for WearableSizeSystemEuropeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableSizeSystemEuropeIri || *other == WEARABLE_SIZE_SYSTEM_EUROPE_LABEL
	}
}
impl PartialEq<WearableSizeSystemEuropeIriOrLabel> for &str {
	fn eq(&self, other: &WearableSizeSystemEuropeIriOrLabel) -> bool {
		*self == WearableSizeSystemEuropeIri || *self == WEARABLE_SIZE_SYSTEM_EUROPE_LABEL
	}
}
