/// <https://schema.org/BodyMeasurementWeight>
pub const BODY_MEASUREMENT_WEIGHT_IRI_HTTP: &str = "http://schema.org/BodyMeasurementWeight";
/// <https://schema.org/BodyMeasurementWeight>
pub const BODY_MEASUREMENT_WEIGHT_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementWeight";
/// <https://schema.org/BodyMeasurementWeight>
pub const BODY_MEASUREMENT_WEIGHT_LABEL: &str = "BodyMeasurementWeight";
pub struct BodyMeasurementWeightIri;
impl PartialEq<&str> for BodyMeasurementWeightIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_WEIGHT_IRI_HTTP || *other == BODY_MEASUREMENT_WEIGHT_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementWeightIri> for &str {
	fn eq(&self, other: &BodyMeasurementWeightIri) -> bool {
		*self == BODY_MEASUREMENT_WEIGHT_IRI_HTTP || *self == BODY_MEASUREMENT_WEIGHT_IRI_HTTPS
	}
}
pub struct BodyMeasurementWeightIriOrLabel;
impl PartialEq<&str> for BodyMeasurementWeightIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementWeightIri || *other == BODY_MEASUREMENT_WEIGHT_LABEL
	}
}
impl PartialEq<BodyMeasurementWeightIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementWeightIriOrLabel) -> bool {
		*self == BodyMeasurementWeightIri || *self == BODY_MEASUREMENT_WEIGHT_LABEL
	}
}
