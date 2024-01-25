/// <https://schema.org/GenericWebPlatform>
pub const GENERIC_WEB_PLATFORM_IRI_HTTP: &str = "http://schema.org/GenericWebPlatform";
/// <https://schema.org/GenericWebPlatform>
pub const GENERIC_WEB_PLATFORM_IRI_HTTPS: &str = "https://schema.org/GenericWebPlatform";
/// <https://schema.org/GenericWebPlatform>
pub const GENERIC_WEB_PLATFORM_LABEL: &str = "GenericWebPlatform";
pub struct GenericWebPlatformIri;
impl PartialEq<&str> for GenericWebPlatformIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENERIC_WEB_PLATFORM_IRI_HTTP || *other == GENERIC_WEB_PLATFORM_IRI_HTTPS
	}
}
impl PartialEq<GenericWebPlatformIri> for &str {
	fn eq(&self, other: &GenericWebPlatformIri) -> bool {
		*self == GENERIC_WEB_PLATFORM_IRI_HTTP || *self == GENERIC_WEB_PLATFORM_IRI_HTTPS
	}
}
pub struct GenericWebPlatformIriOrLabel;
impl PartialEq<&str> for GenericWebPlatformIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GenericWebPlatformIri || *other == GENERIC_WEB_PLATFORM_LABEL
	}
}
impl PartialEq<GenericWebPlatformIriOrLabel> for &str {
	fn eq(&self, other: &GenericWebPlatformIriOrLabel) -> bool {
		*self == GenericWebPlatformIri || *self == GENERIC_WEB_PLATFORM_LABEL
	}
}
