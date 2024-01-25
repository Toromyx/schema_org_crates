/// <https://schema.org/WearableMeasurementHeight>
pub const WEARABLE_MEASUREMENT_HEIGHT_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementHeight";
/// <https://schema.org/WearableMeasurementHeight>
pub const WEARABLE_MEASUREMENT_HEIGHT_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementHeight";
/// <https://schema.org/WearableMeasurementHeight>
pub const WEARABLE_MEASUREMENT_HEIGHT_LABEL: &str = "WearableMeasurementHeight";
pub struct WearableMeasurementHeightIri;
impl PartialEq<&str> for WearableMeasurementHeightIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_HEIGHT_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_HEIGHT_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementHeightIri> for &str {
	fn eq(&self, other: &WearableMeasurementHeightIri) -> bool {
		*self == WEARABLE_MEASUREMENT_HEIGHT_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_HEIGHT_IRI_HTTPS
	}
}
pub struct WearableMeasurementHeightIriOrLabel;
impl PartialEq<&str> for WearableMeasurementHeightIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementHeightIri || *other == WEARABLE_MEASUREMENT_HEIGHT_LABEL
	}
}
impl PartialEq<WearableMeasurementHeightIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementHeightIriOrLabel) -> bool {
		*self == WearableMeasurementHeightIri || *self == WEARABLE_MEASUREMENT_HEIGHT_LABEL
	}
}
