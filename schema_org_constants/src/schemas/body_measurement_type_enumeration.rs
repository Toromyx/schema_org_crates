/// <https://schema.org/BodyMeasurementTypeEnumeration>
pub const BODY_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/BodyMeasurementTypeEnumeration";
/// <https://schema.org/BodyMeasurementTypeEnumeration>
pub const BODY_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/BodyMeasurementTypeEnumeration";
/// <https://schema.org/BodyMeasurementTypeEnumeration>
pub const BODY_MEASUREMENT_TYPE_ENUMERATION_LABEL: &str = "BodyMeasurementTypeEnumeration";
pub struct BodyMeasurementTypeEnumerationIri;
impl PartialEq<&str> for BodyMeasurementTypeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
			|| *other == BODY_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementTypeEnumerationIri> for &str {
	fn eq(&self, other: &BodyMeasurementTypeEnumerationIri) -> bool {
		*self == BODY_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
			|| *self == BODY_MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
pub struct BodyMeasurementTypeEnumerationIriOrLabel;
impl PartialEq<&str> for BodyMeasurementTypeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementTypeEnumerationIri
			|| *other == BODY_MEASUREMENT_TYPE_ENUMERATION_LABEL
	}
}
impl PartialEq<BodyMeasurementTypeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementTypeEnumerationIriOrLabel) -> bool {
		*self == BodyMeasurementTypeEnumerationIri
			|| *self == BODY_MEASUREMENT_TYPE_ENUMERATION_LABEL
	}
}
