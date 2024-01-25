/// <https://schema.org/SteeringPositionValue>
pub const STEERING_POSITION_VALUE_IRI_HTTP: &str = "http://schema.org/SteeringPositionValue";
/// <https://schema.org/SteeringPositionValue>
pub const STEERING_POSITION_VALUE_IRI_HTTPS: &str = "https://schema.org/SteeringPositionValue";
/// <https://schema.org/SteeringPositionValue>
pub const STEERING_POSITION_VALUE_LABEL: &str = "SteeringPositionValue";
pub struct SteeringPositionValueIri;
impl PartialEq<&str> for SteeringPositionValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STEERING_POSITION_VALUE_IRI_HTTP || *other == STEERING_POSITION_VALUE_IRI_HTTPS
	}
}
impl PartialEq<SteeringPositionValueIri> for &str {
	fn eq(&self, other: &SteeringPositionValueIri) -> bool {
		*self == STEERING_POSITION_VALUE_IRI_HTTP || *self == STEERING_POSITION_VALUE_IRI_HTTPS
	}
}
pub struct SteeringPositionValueIriOrLabel;
impl PartialEq<&str> for SteeringPositionValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SteeringPositionValueIri || *other == STEERING_POSITION_VALUE_LABEL
	}
}
impl PartialEq<SteeringPositionValueIriOrLabel> for &str {
	fn eq(&self, other: &SteeringPositionValueIriOrLabel) -> bool {
		*self == SteeringPositionValueIri || *self == STEERING_POSITION_VALUE_LABEL
	}
}
