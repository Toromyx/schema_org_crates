/// <https://schema.org/DemoGameAvailability>
pub const DEMO_GAME_AVAILABILITY_IRI_HTTP: &str = "http://schema.org/DemoGameAvailability";
/// <https://schema.org/DemoGameAvailability>
pub const DEMO_GAME_AVAILABILITY_IRI_HTTPS: &str = "https://schema.org/DemoGameAvailability";
/// <https://schema.org/DemoGameAvailability>
pub const DEMO_GAME_AVAILABILITY_LABEL: &str = "DemoGameAvailability";
pub struct DemoGameAvailabilityIri;
impl PartialEq<&str> for DemoGameAvailabilityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEMO_GAME_AVAILABILITY_IRI_HTTP || *other == DEMO_GAME_AVAILABILITY_IRI_HTTPS
	}
}
impl PartialEq<DemoGameAvailabilityIri> for &str {
	fn eq(&self, other: &DemoGameAvailabilityIri) -> bool {
		*self == DEMO_GAME_AVAILABILITY_IRI_HTTP || *self == DEMO_GAME_AVAILABILITY_IRI_HTTPS
	}
}
pub struct DemoGameAvailabilityIriOrLabel;
impl PartialEq<&str> for DemoGameAvailabilityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DemoGameAvailabilityIri || *other == DEMO_GAME_AVAILABILITY_LABEL
	}
}
impl PartialEq<DemoGameAvailabilityIriOrLabel> for &str {
	fn eq(&self, other: &DemoGameAvailabilityIriOrLabel) -> bool {
		*self == DemoGameAvailabilityIri || *self == DEMO_GAME_AVAILABILITY_LABEL
	}
}
