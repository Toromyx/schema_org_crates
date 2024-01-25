/// <https://schema.org/BodyMeasurementNeck>
pub const BODY_MEASUREMENT_NECK_IRI_HTTP: &str = "http://schema.org/BodyMeasurementNeck";
/// <https://schema.org/BodyMeasurementNeck>
pub const BODY_MEASUREMENT_NECK_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementNeck";
/// <https://schema.org/BodyMeasurementNeck>
pub const BODY_MEASUREMENT_NECK_LABEL: &str = "BodyMeasurementNeck";
pub struct BodyMeasurementNeckIri;
impl PartialEq<&str> for BodyMeasurementNeckIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_NECK_IRI_HTTP || *other == BODY_MEASUREMENT_NECK_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementNeckIri> for &str {
	fn eq(&self, other: &BodyMeasurementNeckIri) -> bool {
		*self == BODY_MEASUREMENT_NECK_IRI_HTTP || *self == BODY_MEASUREMENT_NECK_IRI_HTTPS
	}
}
pub struct BodyMeasurementNeckIriOrLabel;
impl PartialEq<&str> for BodyMeasurementNeckIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementNeckIri || *other == BODY_MEASUREMENT_NECK_LABEL
	}
}
impl PartialEq<BodyMeasurementNeckIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementNeckIriOrLabel) -> bool {
		*self == BodyMeasurementNeckIri || *self == BODY_MEASUREMENT_NECK_LABEL
	}
}
