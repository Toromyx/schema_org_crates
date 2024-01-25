/// <https://schema.org/MeasurementTypeEnumeration>
pub const MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/MeasurementTypeEnumeration";
/// <https://schema.org/MeasurementTypeEnumeration>
pub const MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/MeasurementTypeEnumeration";
/// <https://schema.org/MeasurementTypeEnumeration>
pub const MEASUREMENT_TYPE_ENUMERATION_LABEL: &str = "MeasurementTypeEnumeration";
pub struct MeasurementTypeEnumerationIri;
impl PartialEq<&str> for MeasurementTypeEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
			|| *other == MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<MeasurementTypeEnumerationIri> for &str {
	fn eq(&self, other: &MeasurementTypeEnumerationIri) -> bool {
		*self == MEASUREMENT_TYPE_ENUMERATION_IRI_HTTP
			|| *self == MEASUREMENT_TYPE_ENUMERATION_IRI_HTTPS
	}
}
pub struct MeasurementTypeEnumerationIriOrLabel;
impl PartialEq<&str> for MeasurementTypeEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasurementTypeEnumerationIri || *other == MEASUREMENT_TYPE_ENUMERATION_LABEL
	}
}
impl PartialEq<MeasurementTypeEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &MeasurementTypeEnumerationIriOrLabel) -> bool {
		*self == MeasurementTypeEnumerationIri || *self == MEASUREMENT_TYPE_ENUMERATION_LABEL
	}
}
