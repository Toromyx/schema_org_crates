/// <https://schema.org/BodyMeasurementFoot>
pub const BODY_MEASUREMENT_FOOT_IRI_HTTP: &str = "http://schema.org/BodyMeasurementFoot";
/// <https://schema.org/BodyMeasurementFoot>
pub const BODY_MEASUREMENT_FOOT_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementFoot";
/// <https://schema.org/BodyMeasurementFoot>
pub const BODY_MEASUREMENT_FOOT_LABEL: &str = "BodyMeasurementFoot";
pub struct BodyMeasurementFootIri;
impl PartialEq<&str> for BodyMeasurementFootIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_FOOT_IRI_HTTP || *other == BODY_MEASUREMENT_FOOT_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementFootIri> for &str {
	fn eq(&self, other: &BodyMeasurementFootIri) -> bool {
		*self == BODY_MEASUREMENT_FOOT_IRI_HTTP || *self == BODY_MEASUREMENT_FOOT_IRI_HTTPS
	}
}
pub struct BodyMeasurementFootIriOrLabel;
impl PartialEq<&str> for BodyMeasurementFootIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementFootIri || *other == BODY_MEASUREMENT_FOOT_LABEL
	}
}
impl PartialEq<BodyMeasurementFootIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementFootIriOrLabel) -> bool {
		*self == BodyMeasurementFootIri || *self == BODY_MEASUREMENT_FOOT_LABEL
	}
}
