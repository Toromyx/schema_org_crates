/// <https://schema.org/accelerationTime>
pub const ACCELERATION_TIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/accelerationTime";
/// <https://schema.org/accelerationTime>
pub const ACCELERATION_TIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accelerationTime";
/// <https://schema.org/accelerationTime>
pub const ACCELERATION_TIME_PROPERTY_LABEL: &str = "accelerationTime";
pub struct AccelerationTimePropertyIri;
impl PartialEq<&str> for AccelerationTimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCELERATION_TIME_PROPERTY_IRI_HTTP
			|| *other == ACCELERATION_TIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccelerationTimePropertyIri> for &str {
	fn eq(&self, other: &AccelerationTimePropertyIri) -> bool {
		*self == ACCELERATION_TIME_PROPERTY_IRI_HTTP
			|| *self == ACCELERATION_TIME_PROPERTY_IRI_HTTPS
	}
}
pub struct AccelerationTimePropertyIriOrLabel;
impl PartialEq<&str> for AccelerationTimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccelerationTimePropertyIri || *other == ACCELERATION_TIME_PROPERTY_LABEL
	}
}
impl PartialEq<AccelerationTimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccelerationTimePropertyIriOrLabel) -> bool {
		*self == AccelerationTimePropertyIri || *self == ACCELERATION_TIME_PROPERTY_LABEL
	}
}
