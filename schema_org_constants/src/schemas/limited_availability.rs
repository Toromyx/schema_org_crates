/// <https://schema.org/LimitedAvailability>
pub const LIMITED_AVAILABILITY_IRI_HTTP: &str = "http://schema.org/LimitedAvailability";
/// <https://schema.org/LimitedAvailability>
pub const LIMITED_AVAILABILITY_IRI_HTTPS: &str = "https://schema.org/LimitedAvailability";
/// <https://schema.org/LimitedAvailability>
pub const LIMITED_AVAILABILITY_LABEL: &str = "LimitedAvailability";
pub struct LimitedAvailabilityIri;
impl PartialEq<&str> for LimitedAvailabilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIMITED_AVAILABILITY_IRI_HTTP || *other == LIMITED_AVAILABILITY_IRI_HTTPS
	}
}
impl PartialEq<LimitedAvailabilityIri> for &str {
	fn eq(&self, other: &LimitedAvailabilityIri) -> bool {
		*self == LIMITED_AVAILABILITY_IRI_HTTP || *self == LIMITED_AVAILABILITY_IRI_HTTPS
	}
}
pub struct LimitedAvailabilityIriOrLabel;
impl PartialEq<&str> for LimitedAvailabilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LimitedAvailabilityIri || *other == LIMITED_AVAILABILITY_LABEL
	}
}
impl PartialEq<LimitedAvailabilityIriOrLabel> for &str {
	fn eq(&self, other: &LimitedAvailabilityIriOrLabel) -> bool {
		*self == LimitedAvailabilityIri || *self == LIMITED_AVAILABILITY_LABEL
	}
}
