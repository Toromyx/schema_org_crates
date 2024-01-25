/// <https://schema.org/WearableMeasurementWaist>
pub const WEARABLE_MEASUREMENT_WAIST_IRI_HTTP: &str = "http://schema.org/WearableMeasurementWaist";
/// <https://schema.org/WearableMeasurementWaist>
pub const WEARABLE_MEASUREMENT_WAIST_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementWaist";
/// <https://schema.org/WearableMeasurementWaist>
pub const WEARABLE_MEASUREMENT_WAIST_LABEL: &str = "WearableMeasurementWaist";
pub struct WearableMeasurementWaistIri;
impl PartialEq<&str> for WearableMeasurementWaistIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_WAIST_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_WAIST_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementWaistIri> for &str {
	fn eq(&self, other: &WearableMeasurementWaistIri) -> bool {
		*self == WEARABLE_MEASUREMENT_WAIST_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_WAIST_IRI_HTTPS
	}
}
pub struct WearableMeasurementWaistIriOrLabel;
impl PartialEq<&str> for WearableMeasurementWaistIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementWaistIri || *other == WEARABLE_MEASUREMENT_WAIST_LABEL
	}
}
impl PartialEq<WearableMeasurementWaistIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementWaistIriOrLabel) -> bool {
		*self == WearableMeasurementWaistIri || *self == WEARABLE_MEASUREMENT_WAIST_LABEL
	}
}
