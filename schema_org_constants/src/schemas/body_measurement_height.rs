/// <https://schema.org/BodyMeasurementHeight>
pub const BODY_MEASUREMENT_HEIGHT_IRI_HTTP: &str = "http://schema.org/BodyMeasurementHeight";
/// <https://schema.org/BodyMeasurementHeight>
pub const BODY_MEASUREMENT_HEIGHT_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementHeight";
/// <https://schema.org/BodyMeasurementHeight>
pub const BODY_MEASUREMENT_HEIGHT_LABEL: &str = "BodyMeasurementHeight";
pub struct BodyMeasurementHeightIri;
impl PartialEq<&str> for BodyMeasurementHeightIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_HEIGHT_IRI_HTTP || *other == BODY_MEASUREMENT_HEIGHT_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementHeightIri> for &str {
	fn eq(&self, other: &BodyMeasurementHeightIri) -> bool {
		*self == BODY_MEASUREMENT_HEIGHT_IRI_HTTP || *self == BODY_MEASUREMENT_HEIGHT_IRI_HTTPS
	}
}
pub struct BodyMeasurementHeightIriOrLabel;
impl PartialEq<&str> for BodyMeasurementHeightIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementHeightIri || *other == BODY_MEASUREMENT_HEIGHT_LABEL
	}
}
impl PartialEq<BodyMeasurementHeightIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementHeightIriOrLabel) -> bool {
		*self == BodyMeasurementHeightIri || *self == BODY_MEASUREMENT_HEIGHT_LABEL
	}
}
