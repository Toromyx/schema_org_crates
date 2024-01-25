/// <https://schema.org/WearableMeasurementBack>
pub const WEARABLE_MEASUREMENT_BACK_IRI_HTTP: &str = "http://schema.org/WearableMeasurementBack";
/// <https://schema.org/WearableMeasurementBack>
pub const WEARABLE_MEASUREMENT_BACK_IRI_HTTPS: &str = "https://schema.org/WearableMeasurementBack";
/// <https://schema.org/WearableMeasurementBack>
pub const WEARABLE_MEASUREMENT_BACK_LABEL: &str = "WearableMeasurementBack";
pub struct WearableMeasurementBackIri;
impl PartialEq<&str> for WearableMeasurementBackIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_BACK_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_BACK_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementBackIri> for &str {
	fn eq(&self, other: &WearableMeasurementBackIri) -> bool {
		*self == WEARABLE_MEASUREMENT_BACK_IRI_HTTP || *self == WEARABLE_MEASUREMENT_BACK_IRI_HTTPS
	}
}
pub struct WearableMeasurementBackIriOrLabel;
impl PartialEq<&str> for WearableMeasurementBackIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementBackIri || *other == WEARABLE_MEASUREMENT_BACK_LABEL
	}
}
impl PartialEq<WearableMeasurementBackIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementBackIriOrLabel) -> bool {
		*self == WearableMeasurementBackIri || *self == WEARABLE_MEASUREMENT_BACK_LABEL
	}
}
