/// <https://schema.org/WearableMeasurementCollar>
pub const WEARABLE_MEASUREMENT_COLLAR_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementCollar";
/// <https://schema.org/WearableMeasurementCollar>
pub const WEARABLE_MEASUREMENT_COLLAR_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementCollar";
/// <https://schema.org/WearableMeasurementCollar>
pub const WEARABLE_MEASUREMENT_COLLAR_LABEL: &str = "WearableMeasurementCollar";
pub struct WearableMeasurementCollarIri;
impl PartialEq<&str> for WearableMeasurementCollarIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_COLLAR_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_COLLAR_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementCollarIri> for &str {
	fn eq(&self, other: &WearableMeasurementCollarIri) -> bool {
		*self == WEARABLE_MEASUREMENT_COLLAR_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_COLLAR_IRI_HTTPS
	}
}
pub struct WearableMeasurementCollarIriOrLabel;
impl PartialEq<&str> for WearableMeasurementCollarIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementCollarIri || *other == WEARABLE_MEASUREMENT_COLLAR_LABEL
	}
}
impl PartialEq<WearableMeasurementCollarIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementCollarIriOrLabel) -> bool {
		*self == WearableMeasurementCollarIri || *self == WEARABLE_MEASUREMENT_COLLAR_LABEL
	}
}
