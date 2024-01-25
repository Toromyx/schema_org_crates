/// <https://schema.org/WearableMeasurementCup>
pub const WEARABLE_MEASUREMENT_CUP_IRI_HTTP: &str = "http://schema.org/WearableMeasurementCup";
/// <https://schema.org/WearableMeasurementCup>
pub const WEARABLE_MEASUREMENT_CUP_IRI_HTTPS: &str = "https://schema.org/WearableMeasurementCup";
/// <https://schema.org/WearableMeasurementCup>
pub const WEARABLE_MEASUREMENT_CUP_LABEL: &str = "WearableMeasurementCup";
pub struct WearableMeasurementCupIri;
impl PartialEq<&str> for WearableMeasurementCupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_CUP_IRI_HTTP || *other == WEARABLE_MEASUREMENT_CUP_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementCupIri> for &str {
	fn eq(&self, other: &WearableMeasurementCupIri) -> bool {
		*self == WEARABLE_MEASUREMENT_CUP_IRI_HTTP || *self == WEARABLE_MEASUREMENT_CUP_IRI_HTTPS
	}
}
pub struct WearableMeasurementCupIriOrLabel;
impl PartialEq<&str> for WearableMeasurementCupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementCupIri || *other == WEARABLE_MEASUREMENT_CUP_LABEL
	}
}
impl PartialEq<WearableMeasurementCupIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementCupIriOrLabel) -> bool {
		*self == WearableMeasurementCupIri || *self == WEARABLE_MEASUREMENT_CUP_LABEL
	}
}
