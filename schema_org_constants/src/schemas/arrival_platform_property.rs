/// <https://schema.org/arrivalPlatform>
pub const ARRIVAL_PLATFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalPlatform";
/// <https://schema.org/arrivalPlatform>
pub const ARRIVAL_PLATFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalPlatform";
/// <https://schema.org/arrivalPlatform>
pub const ARRIVAL_PLATFORM_PROPERTY_LABEL: &str = "arrivalPlatform";
pub struct ArrivalPlatformPropertyIri;
impl PartialEq<&str> for ArrivalPlatformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_PLATFORM_PROPERTY_IRI_HTTP
			|| *other == ARRIVAL_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalPlatformPropertyIri> for &str {
	fn eq(&self, other: &ArrivalPlatformPropertyIri) -> bool {
		*self == ARRIVAL_PLATFORM_PROPERTY_IRI_HTTP || *self == ARRIVAL_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalPlatformPropertyIriOrLabel;
impl PartialEq<&str> for ArrivalPlatformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalPlatformPropertyIri || *other == ARRIVAL_PLATFORM_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalPlatformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalPlatformPropertyIriOrLabel) -> bool {
		*self == ArrivalPlatformPropertyIri || *self == ARRIVAL_PLATFORM_PROPERTY_LABEL
	}
}
