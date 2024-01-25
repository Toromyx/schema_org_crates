/// <https://schema.org/BodyMeasurementHead>
pub const BODY_MEASUREMENT_HEAD_IRI_HTTP: &str = "http://schema.org/BodyMeasurementHead";
/// <https://schema.org/BodyMeasurementHead>
pub const BODY_MEASUREMENT_HEAD_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementHead";
/// <https://schema.org/BodyMeasurementHead>
pub const BODY_MEASUREMENT_HEAD_LABEL: &str = "BodyMeasurementHead";
pub struct BodyMeasurementHeadIri;
impl PartialEq<&str> for BodyMeasurementHeadIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_HEAD_IRI_HTTP || *other == BODY_MEASUREMENT_HEAD_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementHeadIri> for &str {
	fn eq(&self, other: &BodyMeasurementHeadIri) -> bool {
		*self == BODY_MEASUREMENT_HEAD_IRI_HTTP || *self == BODY_MEASUREMENT_HEAD_IRI_HTTPS
	}
}
pub struct BodyMeasurementHeadIriOrLabel;
impl PartialEq<&str> for BodyMeasurementHeadIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementHeadIri || *other == BODY_MEASUREMENT_HEAD_LABEL
	}
}
impl PartialEq<BodyMeasurementHeadIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementHeadIriOrLabel) -> bool {
		*self == BodyMeasurementHeadIri || *self == BODY_MEASUREMENT_HEAD_LABEL
	}
}
