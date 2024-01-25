/// <https://schema.org/BodyMeasurementWaist>
pub const BODY_MEASUREMENT_WAIST_IRI_HTTP: &str = "http://schema.org/BodyMeasurementWaist";
/// <https://schema.org/BodyMeasurementWaist>
pub const BODY_MEASUREMENT_WAIST_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementWaist";
/// <https://schema.org/BodyMeasurementWaist>
pub const BODY_MEASUREMENT_WAIST_LABEL: &str = "BodyMeasurementWaist";
pub struct BodyMeasurementWaistIri;
impl PartialEq<&str> for BodyMeasurementWaistIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_WAIST_IRI_HTTP || *other == BODY_MEASUREMENT_WAIST_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementWaistIri> for &str {
	fn eq(&self, other: &BodyMeasurementWaistIri) -> bool {
		*self == BODY_MEASUREMENT_WAIST_IRI_HTTP || *self == BODY_MEASUREMENT_WAIST_IRI_HTTPS
	}
}
pub struct BodyMeasurementWaistIriOrLabel;
impl PartialEq<&str> for BodyMeasurementWaistIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementWaistIri || *other == BODY_MEASUREMENT_WAIST_LABEL
	}
}
impl PartialEq<BodyMeasurementWaistIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementWaistIriOrLabel) -> bool {
		*self == BodyMeasurementWaistIri || *self == BODY_MEASUREMENT_WAIST_LABEL
	}
}
