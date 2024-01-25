/// <https://schema.org/WearableMeasurementWidth>
pub const WEARABLE_MEASUREMENT_WIDTH_IRI_HTTP: &str = "http://schema.org/WearableMeasurementWidth";
/// <https://schema.org/WearableMeasurementWidth>
pub const WEARABLE_MEASUREMENT_WIDTH_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementWidth";
/// <https://schema.org/WearableMeasurementWidth>
pub const WEARABLE_MEASUREMENT_WIDTH_LABEL: &str = "WearableMeasurementWidth";
pub struct WearableMeasurementWidthIri;
impl PartialEq<&str> for WearableMeasurementWidthIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_WIDTH_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_WIDTH_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementWidthIri> for &str {
	fn eq(&self, other: &WearableMeasurementWidthIri) -> bool {
		*self == WEARABLE_MEASUREMENT_WIDTH_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_WIDTH_IRI_HTTPS
	}
}
pub struct WearableMeasurementWidthIriOrLabel;
impl PartialEq<&str> for WearableMeasurementWidthIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementWidthIri || *other == WEARABLE_MEASUREMENT_WIDTH_LABEL
	}
}
impl PartialEq<WearableMeasurementWidthIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementWidthIriOrLabel) -> bool {
		*self == WearableMeasurementWidthIri || *self == WEARABLE_MEASUREMENT_WIDTH_LABEL
	}
}
