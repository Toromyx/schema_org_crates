/// <https://schema.org/WearableMeasurementSleeve>
pub const WEARABLE_MEASUREMENT_SLEEVE_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementSleeve";
/// <https://schema.org/WearableMeasurementSleeve>
pub const WEARABLE_MEASUREMENT_SLEEVE_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementSleeve";
/// <https://schema.org/WearableMeasurementSleeve>
pub const WEARABLE_MEASUREMENT_SLEEVE_LABEL: &str = "WearableMeasurementSleeve";
pub struct WearableMeasurementSleeveIri;
impl PartialEq<&str> for WearableMeasurementSleeveIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_SLEEVE_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_SLEEVE_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementSleeveIri> for &str {
	fn eq(&self, other: &WearableMeasurementSleeveIri) -> bool {
		*self == WEARABLE_MEASUREMENT_SLEEVE_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_SLEEVE_IRI_HTTPS
	}
}
pub struct WearableMeasurementSleeveIriOrLabel;
impl PartialEq<&str> for WearableMeasurementSleeveIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementSleeveIri || *other == WEARABLE_MEASUREMENT_SLEEVE_LABEL
	}
}
impl PartialEq<WearableMeasurementSleeveIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementSleeveIriOrLabel) -> bool {
		*self == WearableMeasurementSleeveIri || *self == WEARABLE_MEASUREMENT_SLEEVE_LABEL
	}
}
