/// <https://schema.org/WearableMeasurementLength>
pub const WEARABLE_MEASUREMENT_LENGTH_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementLength";
/// <https://schema.org/WearableMeasurementLength>
pub const WEARABLE_MEASUREMENT_LENGTH_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementLength";
/// <https://schema.org/WearableMeasurementLength>
pub const WEARABLE_MEASUREMENT_LENGTH_LABEL: &str = "WearableMeasurementLength";
pub struct WearableMeasurementLengthIri;
impl PartialEq<&str> for WearableMeasurementLengthIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_LENGTH_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_LENGTH_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementLengthIri> for &str {
	fn eq(&self, other: &WearableMeasurementLengthIri) -> bool {
		*self == WEARABLE_MEASUREMENT_LENGTH_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_LENGTH_IRI_HTTPS
	}
}
pub struct WearableMeasurementLengthIriOrLabel;
impl PartialEq<&str> for WearableMeasurementLengthIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementLengthIri || *other == WEARABLE_MEASUREMENT_LENGTH_LABEL
	}
}
impl PartialEq<WearableMeasurementLengthIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementLengthIriOrLabel) -> bool {
		*self == WearableMeasurementLengthIri || *self == WEARABLE_MEASUREMENT_LENGTH_LABEL
	}
}
