/// <https://schema.org/BodyMeasurementArm>
pub const BODY_MEASUREMENT_ARM_IRI_HTTP: &str = "http://schema.org/BodyMeasurementArm";
/// <https://schema.org/BodyMeasurementArm>
pub const BODY_MEASUREMENT_ARM_IRI_HTTPS: &str = "https://schema.org/BodyMeasurementArm";
/// <https://schema.org/BodyMeasurementArm>
pub const BODY_MEASUREMENT_ARM_LABEL: &str = "BodyMeasurementArm";
pub struct BodyMeasurementArmIri;
impl PartialEq<&str> for BodyMeasurementArmIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BODY_MEASUREMENT_ARM_IRI_HTTP || *other == BODY_MEASUREMENT_ARM_IRI_HTTPS
	}
}
impl PartialEq<BodyMeasurementArmIri> for &str {
	fn eq(&self, other: &BodyMeasurementArmIri) -> bool {
		*self == BODY_MEASUREMENT_ARM_IRI_HTTP || *self == BODY_MEASUREMENT_ARM_IRI_HTTPS
	}
}
pub struct BodyMeasurementArmIriOrLabel;
impl PartialEq<&str> for BodyMeasurementArmIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BodyMeasurementArmIri || *other == BODY_MEASUREMENT_ARM_LABEL
	}
}
impl PartialEq<BodyMeasurementArmIriOrLabel> for &str {
	fn eq(&self, other: &BodyMeasurementArmIriOrLabel) -> bool {
		*self == BodyMeasurementArmIri || *self == BODY_MEASUREMENT_ARM_LABEL
	}
}
