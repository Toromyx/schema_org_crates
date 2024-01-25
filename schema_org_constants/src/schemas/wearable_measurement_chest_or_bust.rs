/// <https://schema.org/WearableMeasurementChestOrBust>
pub const WEARABLE_MEASUREMENT_CHEST_OR_BUST_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementChestOrBust";
/// <https://schema.org/WearableMeasurementChestOrBust>
pub const WEARABLE_MEASUREMENT_CHEST_OR_BUST_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementChestOrBust";
/// <https://schema.org/WearableMeasurementChestOrBust>
pub const WEARABLE_MEASUREMENT_CHEST_OR_BUST_LABEL: &str = "WearableMeasurementChestOrBust";
pub struct WearableMeasurementChestOrBustIri;
impl PartialEq<&str> for WearableMeasurementChestOrBustIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_CHEST_OR_BUST_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_CHEST_OR_BUST_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementChestOrBustIri> for &str {
	fn eq(&self, other: &WearableMeasurementChestOrBustIri) -> bool {
		*self == WEARABLE_MEASUREMENT_CHEST_OR_BUST_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_CHEST_OR_BUST_IRI_HTTPS
	}
}
pub struct WearableMeasurementChestOrBustIriOrLabel;
impl PartialEq<&str> for WearableMeasurementChestOrBustIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementChestOrBustIri
			|| *other == WEARABLE_MEASUREMENT_CHEST_OR_BUST_LABEL
	}
}
impl PartialEq<WearableMeasurementChestOrBustIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementChestOrBustIriOrLabel) -> bool {
		*self == WearableMeasurementChestOrBustIri
			|| *self == WEARABLE_MEASUREMENT_CHEST_OR_BUST_LABEL
	}
}
