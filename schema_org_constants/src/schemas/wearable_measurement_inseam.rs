/// <https://schema.org/WearableMeasurementInseam>
pub const WEARABLE_MEASUREMENT_INSEAM_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementInseam";
/// <https://schema.org/WearableMeasurementInseam>
pub const WEARABLE_MEASUREMENT_INSEAM_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementInseam";
/// <https://schema.org/WearableMeasurementInseam>
pub const WEARABLE_MEASUREMENT_INSEAM_LABEL: &str = "WearableMeasurementInseam";
pub struct WearableMeasurementInseamIri;
impl PartialEq<&str> for WearableMeasurementInseamIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_INSEAM_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_INSEAM_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementInseamIri> for &str {
	fn eq(&self, other: &WearableMeasurementInseamIri) -> bool {
		*self == WEARABLE_MEASUREMENT_INSEAM_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_INSEAM_IRI_HTTPS
	}
}
pub struct WearableMeasurementInseamIriOrLabel;
impl PartialEq<&str> for WearableMeasurementInseamIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementInseamIri || *other == WEARABLE_MEASUREMENT_INSEAM_LABEL
	}
}
impl PartialEq<WearableMeasurementInseamIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementInseamIriOrLabel) -> bool {
		*self == WearableMeasurementInseamIri || *self == WEARABLE_MEASUREMENT_INSEAM_LABEL
	}
}
