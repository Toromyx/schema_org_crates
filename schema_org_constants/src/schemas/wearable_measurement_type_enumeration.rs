/// <https://schema.org/WearableMeasurementTypeEnumeration>
pub const WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/WearableMeasurementTypeEnumeration";
/// <https://schema.org/WearableMeasurementTypeEnumeration>
pub const WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/WearableMeasurementTypeEnumeration";
/// <https://schema.org/WearableMeasurementTypeEnumeration>
pub const WEARABLE_MEASUREMENT_TYPE_ENUMERATION_LABEL: &str = "WearableMeasurementTypeEnumeration";
pub struct WearableMeasurementTypeEnumerationIri;
impl PartialEq<&str> for WearableMeasurementTypeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
			|| *other == WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<WearableMeasurementTypeEnumerationIri> for &str {
	fn eq(&self, other: &WearableMeasurementTypeEnumerationIri) -> bool {
		*self == WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
			|| *self == WEARABLE_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
pub struct WearableMeasurementTypeEnumerationIriOrLabel;
impl PartialEq<&str> for WearableMeasurementTypeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearableMeasurementTypeEnumerationIri
			|| *other == WEARABLE_MEASUREMENT_TYPE_ENUMERATION_LABEL
	}
}
impl PartialEq<WearableMeasurementTypeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &WearableMeasurementTypeEnumerationIriOrLabel) -> bool {
		*self == WearableMeasurementTypeEnumerationIri
			|| *self == WEARABLE_MEASUREMENT_TYPE_ENUMERATION_LABEL
	}
}
