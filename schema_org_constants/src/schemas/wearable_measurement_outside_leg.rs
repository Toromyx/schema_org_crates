/// <https://schema.org/WearableMeasurementOutsideLeg>
pub const WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementOutsideLeg";
/// <https://schema.org/WearableMeasurementOutsideLeg>
pub const WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementOutsideLeg";
/// <https://schema.org/WearableMeasurementOutsideLeg>
pub const WEARABLE_MEASUREMENT_OUTSIDE_LEG_LABEL: &str = "WearableMeasurementOutsideLeg";
pub struct WearableMeasurementOutsideLegIri;
impl PartialEq<&str> for WearableMeasurementOutsideLegIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementOutsideLegIri> for &str {
	fn eq(&self, other: &WearableMeasurementOutsideLegIri) -> bool {
		*self == WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_OUTSIDE_LEG_IRI_HTTPS
	}
}
pub struct WearableMeasurementOutsideLegIriOrLabel;
impl PartialEq<&str> for WearableMeasurementOutsideLegIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementOutsideLegIri
			|| *other == WEARABLE_MEASUREMENT_OUTSIDE_LEG_LABEL
	}
}
impl PartialEq<WearableMeasurementOutsideLegIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementOutsideLegIriOrLabel) -> bool {
		*self == WearableMeasurementOutsideLegIri || *self == WEARABLE_MEASUREMENT_OUTSIDE_LEG_LABEL
	}
}
