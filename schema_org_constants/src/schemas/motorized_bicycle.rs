/// <https://schema.org/MotorizedBicycle>
pub const MOTORIZED_BICYCLE_IRI_HTTP: &str = "http://schema.org/MotorizedBicycle";
/// <https://schema.org/MotorizedBicycle>
pub const MOTORIZED_BICYCLE_IRI_HTTPS: &str = "https://schema.org/MotorizedBicycle";
/// <https://schema.org/MotorizedBicycle>
pub const MOTORIZED_BICYCLE_LABEL: &str = "MotorizedBicycle";
pub struct MotorizedBicycleIri;
impl PartialEq<&str> for MotorizedBicycleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOTORIZED_BICYCLE_IRI_HTTP || *other == MOTORIZED_BICYCLE_IRI_HTTPS
	}
}
impl PartialEq<MotorizedBicycleIri> for &str {
	fn eq(&self, other: &MotorizedBicycleIri) -> bool {
		*self == MOTORIZED_BICYCLE_IRI_HTTP || *self == MOTORIZED_BICYCLE_IRI_HTTPS
	}
}
pub struct MotorizedBicycleIriOrLabel;
impl PartialEq<&str> for MotorizedBicycleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MotorizedBicycleIri || *other == MOTORIZED_BICYCLE_LABEL
	}
}
impl PartialEq<MotorizedBicycleIriOrLabel> for &str {
	fn eq(&self, other: &MotorizedBicycleIriOrLabel) -> bool {
		*self == MotorizedBicycleIri || *self == MOTORIZED_BICYCLE_LABEL
	}
}
