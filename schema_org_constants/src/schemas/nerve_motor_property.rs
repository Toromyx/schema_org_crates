/// <https://schema.org/nerveMotor>
pub const NERVE_MOTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/nerveMotor";
/// <https://schema.org/nerveMotor>
pub const NERVE_MOTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/nerveMotor";
/// <https://schema.org/nerveMotor>
pub const NERVE_MOTOR_PROPERTY_LABEL: &str = "nerveMotor";
pub struct NerveMotorPropertyIri;
impl PartialEq<&str> for NerveMotorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NERVE_MOTOR_PROPERTY_IRI_HTTP || *other == NERVE_MOTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NerveMotorPropertyIri> for &str {
	fn eq(&self, other: &NerveMotorPropertyIri) -> bool {
		*self == NERVE_MOTOR_PROPERTY_IRI_HTTP || *self == NERVE_MOTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct NerveMotorPropertyIriOrLabel;
impl PartialEq<&str> for NerveMotorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NerveMotorPropertyIri || *other == NERVE_MOTOR_PROPERTY_LABEL
	}
}
impl PartialEq<NerveMotorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NerveMotorPropertyIriOrLabel) -> bool {
		*self == NerveMotorPropertyIri || *self == NERVE_MOTOR_PROPERTY_LABEL
	}
}
