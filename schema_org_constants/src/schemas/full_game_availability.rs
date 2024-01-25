/// <https://schema.org/FullGameAvailability>
pub const FULL_GAME_AVAILABILITY_IRI_HTTP: &str = "http://schema.org/FullGameAvailability";
/// <https://schema.org/FullGameAvailability>
pub const FULL_GAME_AVAILABILITY_IRI_HTTPS: &str = "https://schema.org/FullGameAvailability";
/// <https://schema.org/FullGameAvailability>
pub const FULL_GAME_AVAILABILITY_LABEL: &str = "FullGameAvailability";
pub struct FullGameAvailabilityIri;
impl PartialEq<&str> for FullGameAvailabilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FULL_GAME_AVAILABILITY_IRI_HTTP || *other == FULL_GAME_AVAILABILITY_IRI_HTTPS
	}
}
impl PartialEq<FullGameAvailabilityIri> for &str {
	fn eq(&self, other: &FullGameAvailabilityIri) -> bool {
		*self == FULL_GAME_AVAILABILITY_IRI_HTTP || *self == FULL_GAME_AVAILABILITY_IRI_HTTPS
	}
}
pub struct FullGameAvailabilityIriOrLabel;
impl PartialEq<&str> for FullGameAvailabilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FullGameAvailabilityIri || *other == FULL_GAME_AVAILABILITY_LABEL
	}
}
impl PartialEq<FullGameAvailabilityIriOrLabel> for &str {
	fn eq(&self, other: &FullGameAvailabilityIriOrLabel) -> bool {
		*self == FullGameAvailabilityIri || *self == FULL_GAME_AVAILABILITY_LABEL
	}
}
