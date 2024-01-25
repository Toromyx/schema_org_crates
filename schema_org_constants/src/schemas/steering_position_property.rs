/// <https://schema.org/steeringPosition>
pub const STEERING_POSITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/steeringPosition";
/// <https://schema.org/steeringPosition>
pub const STEERING_POSITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/steeringPosition";
/// <https://schema.org/steeringPosition>
pub const STEERING_POSITION_PROPERTY_LABEL: &str = "steeringPosition";
pub struct SteeringPositionPropertyIri;
impl PartialEq<&str> for SteeringPositionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STEERING_POSITION_PROPERTY_IRI_HTTP
			|| *other == STEERING_POSITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SteeringPositionPropertyIri> for &str {
	fn eq(&self, other: &SteeringPositionPropertyIri) -> bool {
		*self == STEERING_POSITION_PROPERTY_IRI_HTTP
			|| *self == STEERING_POSITION_PROPERTY_IRI_HTTPS
	}
}
pub struct SteeringPositionPropertyIriOrLabel;
impl PartialEq<&str> for SteeringPositionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SteeringPositionPropertyIri || *other == STEERING_POSITION_PROPERTY_LABEL
	}
}
impl PartialEq<SteeringPositionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SteeringPositionPropertyIriOrLabel) -> bool {
		*self == SteeringPositionPropertyIri || *self == STEERING_POSITION_PROPERTY_LABEL
	}
}
