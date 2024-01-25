/// <https://schema.org/BodyMeasurementHand>
pub const BODY_MEASUREMENT_HAND_IRI_HTTP: &str = "http://schema.org/BodyMeasurementHand";
/// <https://schema.org/BodyMeasurementHand>
pub const BODY_MEASUREMENT_HAND_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementHand";
/// <https://schema.org/BodyMeasurementHand>
pub const BODY_MEASUREMENT_HAND_LABEL: &str = "BodyMeasurementHand";
pub struct BodyMeasurementHandIri;
impl PartialEq<&str> for BodyMeasurementHandIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_HAND_IRI_HTTP || *other == BODY_MEASUREMENT_HAND_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementHandIri> for &str {
	fn eq(&self, other: &BodyMeasurementHandIri) -> bool {
		*self == BODY_MEASUREMENT_HAND_IRI_HTTP || *self == BODY_MEASUREMENT_HAND_IRI_HTTPS
	}
}
pub struct BodyMeasurementHandIriOrLabel;
impl PartialEq<&str> for BodyMeasurementHandIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementHandIri || *other == BODY_MEASUREMENT_HAND_LABEL
	}
}
impl PartialEq<BodyMeasurementHandIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementHandIriOrLabel) -> bool {
		*self == BodyMeasurementHandIri || *self == BODY_MEASUREMENT_HAND_LABEL
	}
}
