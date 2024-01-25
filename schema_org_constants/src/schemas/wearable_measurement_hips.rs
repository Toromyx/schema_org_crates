/// <https://schema.org/WearableMeasurementHips>
pub const WEARABLE_MEASUREMENT_HIPS_IRI_HTTP: &str = "http://schema.org/WearableMeasurementHips";
/// <https://schema.org/WearableMeasurementHips>
pub const WEARABLE_MEASUREMENT_HIPS_IRI_HTTPS: &str = "https://schema.org/WearableMeasurementHips";
/// <https://schema.org/WearableMeasurementHips>
pub const WEARABLE_MEASUREMENT_HIPS_LABEL: &str = "WearableMeasurementHips";
pub struct WearableMeasurementHipsIri;
impl PartialEq<&str> for WearableMeasurementHipsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_HIPS_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_HIPS_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementHipsIri> for &str {
	fn eq(&self, other: &WearableMeasurementHipsIri) -> bool {
		*self == WEARABLE_MEASUREMENT_HIPS_IRI_HTTP || *self == WEARABLE_MEASUREMENT_HIPS_IRI_HTTPS
	}
}
pub struct WearableMeasurementHipsIriOrLabel;
impl PartialEq<&str> for WearableMeasurementHipsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementHipsIri || *other == WEARABLE_MEASUREMENT_HIPS_LABEL
	}
}
impl PartialEq<WearableMeasurementHipsIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementHipsIriOrLabel) -> bool {
		*self == WearableMeasurementHipsIri || *self == WEARABLE_MEASUREMENT_HIPS_LABEL
	}
}
