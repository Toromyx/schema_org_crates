/// <https://schema.org/BodyMeasurementBust>
pub const BODY_MEASUREMENT_BUST_IRI_HTTP: &str = "http://schema.org/BodyMeasurementBust";
/// <https://schema.org/BodyMeasurementBust>
pub const BODY_MEASUREMENT_BUST_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementBust";
/// <https://schema.org/BodyMeasurementBust>
pub const BODY_MEASUREMENT_BUST_LABEL: &str = "BodyMeasurementBust";
pub struct BodyMeasurementBustIri;
impl PartialEq<&str> for BodyMeasurementBustIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_BUST_IRI_HTTP || *other == BODY_MEASUREMENT_BUST_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementBustIri> for &str {
	fn eq(&self, other: &BodyMeasurementBustIri) -> bool {
		*self == BODY_MEASUREMENT_BUST_IRI_HTTP || *self == BODY_MEASUREMENT_BUST_IRI_HTTPS
	}
}
pub struct BodyMeasurementBustIriOrLabel;
impl PartialEq<&str> for BodyMeasurementBustIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementBustIri || *other == BODY_MEASUREMENT_BUST_LABEL
	}
}
impl PartialEq<BodyMeasurementBustIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementBustIriOrLabel) -> bool {
		*self == BodyMeasurementBustIri || *self == BODY_MEASUREMENT_BUST_LABEL
	}
}
