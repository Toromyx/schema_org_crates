/// <https://schema.org/BodyMeasurementHips>
pub const BODY_MEASUREMENT_HIPS_IRI_HTTP: &str = "http://schema.org/BodyMeasurementHips";
/// <https://schema.org/BodyMeasurementHips>
pub const BODY_MEASUREMENT_HIPS_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementHips";
/// <https://schema.org/BodyMeasurementHips>
pub const BODY_MEASUREMENT_HIPS_LABEL: &str = "BodyMeasurementHips";
pub struct BodyMeasurementHipsIri;
impl PartialEq<&str> for BodyMeasurementHipsIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_HIPS_IRI_HTTP || *other == BODY_MEASUREMENT_HIPS_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementHipsIri> for &str {
	fn eq(&self, other: &BodyMeasurementHipsIri) -> bool {
		*self == BODY_MEASUREMENT_HIPS_IRI_HTTP || *self == BODY_MEASUREMENT_HIPS_IRI_HTTPS
	}
}
pub struct BodyMeasurementHipsIriOrLabel;
impl PartialEq<&str> for BodyMeasurementHipsIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementHipsIri || *other == BODY_MEASUREMENT_HIPS_LABEL
	}
}
impl PartialEq<BodyMeasurementHipsIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementHipsIriOrLabel) -> bool {
		*self == BodyMeasurementHipsIri || *self == BODY_MEASUREMENT_HIPS_LABEL
	}
}
