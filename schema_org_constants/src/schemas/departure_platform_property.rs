/// <https://schema.org/departurePlatform>
pub const DEPARTURE_PLATFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/departurePlatform";
/// <https://schema.org/departurePlatform>
pub const DEPARTURE_PLATFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departurePlatform";
/// <https://schema.org/departurePlatform>
pub const DEPARTURE_PLATFORM_PROPERTY_LABEL: &str = "departurePlatform";
pub struct DeparturePlatformPropertyIri;
impl PartialEq<&str> for DeparturePlatformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_PLATFORM_PROPERTY_IRI_HTTP
			|| *other == DEPARTURE_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DeparturePlatformPropertyIri> for &str {
	fn eq(&self, other: &DeparturePlatformPropertyIri) -> bool {
		*self == DEPARTURE_PLATFORM_PROPERTY_IRI_HTTP
			|| *self == DEPARTURE_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct DeparturePlatformPropertyIriOrLabel;
impl PartialEq<&str> for DeparturePlatformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DeparturePlatformPropertyIri || *other == DEPARTURE_PLATFORM_PROPERTY_LABEL
	}
}
impl PartialEq<DeparturePlatformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DeparturePlatformPropertyIriOrLabel) -> bool {
		*self == DeparturePlatformPropertyIri || *self == DEPARTURE_PLATFORM_PROPERTY_LABEL
	}
}
