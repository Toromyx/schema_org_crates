/// <https://schema.org/MeasurementMethodEnum>
pub const MEASUREMENT_METHOD_ENUM_IRI_HTTP: &str = "http://schema.org/MeasurementMethodEnum";
/// <https://schema.org/MeasurementMethodEnum>
pub const MEASUREMENT_METHOD_ENUM_IRI_HTTPS: &str = "https://schema.org/MeasurementMethodEnum";
/// <https://schema.org/MeasurementMethodEnum>
pub const MEASUREMENT_METHOD_ENUM_LABEL: &str = "MeasurementMethodEnum";
pub struct MeasurementMethodEnumIri;
impl PartialEq<&str> for MeasurementMethodEnumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASUREMENT_METHOD_ENUM_IRI_HTTP || *other == MEASUREMENT_METHOD_ENUM_IRI_HTTPS
	}
}
impl PartialEq<MeasurementMethodEnumIri> for &str {
	fn eq(&self, other: &MeasurementMethodEnumIri) -> bool {
		*self == MEASUREMENT_METHOD_ENUM_IRI_HTTP || *self == MEASUREMENT_METHOD_ENUM_IRI_HTTPS
	}
}
pub struct MeasurementMethodEnumIriOrLabel;
impl PartialEq<&str> for MeasurementMethodEnumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasurementMethodEnumIri || *other == MEASUREMENT_METHOD_ENUM_LABEL
	}
}
impl PartialEq<MeasurementMethodEnumIriOrLabel> for &str {
	fn eq(&self, other: &MeasurementMethodEnumIriOrLabel) -> bool {
		*self == MeasurementMethodEnumIri || *self == MEASUREMENT_METHOD_ENUM_LABEL
	}
}
