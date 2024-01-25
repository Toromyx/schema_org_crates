/// <https://schema.org/DigitalPlatformEnumeration>
pub const DIGITAL_PLATFORM_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/DigitalPlatformEnumeration";
/// <https://schema.org/DigitalPlatformEnumeration>
pub const DIGITAL_PLATFORM_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/DigitalPlatformEnumeration";
/// <https://schema.org/DigitalPlatformEnumeration>
pub const DIGITAL_PLATFORM_ENUMERATION_LABEL: &str = "DigitalPlatformEnumeration";
pub struct DigitalPlatformEnumerationIri;
impl PartialEq<&str> for DigitalPlatformEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_PLATFORM_ENUMERATION_IRI_HTTP
			|| *other == DIGITAL_PLATFORM_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<DigitalPlatformEnumerationIri> for &str {
	fn eq(&self, other: &DigitalPlatformEnumerationIri) -> bool {
		*self == DIGITAL_PLATFORM_ENUMERATION_IRI_HTTP
			|| *self == DIGITAL_PLATFORM_ENUMERATION_IRI_HTTPS
	}
}
pub struct DigitalPlatformEnumerationIriOrLabel;
impl PartialEq<&str> for DigitalPlatformEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalPlatformEnumerationIri || *other == DIGITAL_PLATFORM_ENUMERATION_LABEL
	}
}
impl PartialEq<DigitalPlatformEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &DigitalPlatformEnumerationIriOrLabel) -> bool {
		*self == DigitalPlatformEnumerationIri || *self == DIGITAL_PLATFORM_ENUMERATION_LABEL
	}
}
