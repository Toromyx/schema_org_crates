/// <https://schema.org/MobileWebPlatform>
pub const MOBILE_WEB_PLATFORM_IRI_HTTP: &str = "http://schema.org/MobileWebPlatform";
/// <https://schema.org/MobileWebPlatform>
pub const MOBILE_WEB_PLATFORM_IRI_HTTPS: &str = "https://schema.org/MobileWebPlatform";
/// <https://schema.org/MobileWebPlatform>
pub const MOBILE_WEB_PLATFORM_LABEL: &str = "MobileWebPlatform";
pub struct MobileWebPlatformIri;
impl PartialEq<&str> for MobileWebPlatformIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOBILE_WEB_PLATFORM_IRI_HTTP || *other == MOBILE_WEB_PLATFORM_IRI_HTTPS
	}
}
impl PartialEq<MobileWebPlatformIri> for &str {
	fn eq(&self, other: &MobileWebPlatformIri) -> bool {
		*self == MOBILE_WEB_PLATFORM_IRI_HTTP || *self == MOBILE_WEB_PLATFORM_IRI_HTTPS
	}
}
pub struct MobileWebPlatformIriOrLabel;
impl PartialEq<&str> for MobileWebPlatformIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MobileWebPlatformIri || *other == MOBILE_WEB_PLATFORM_LABEL
	}
}
impl PartialEq<MobileWebPlatformIriOrLabel> for &str {
	fn eq(&self, other: &MobileWebPlatformIriOrLabel) -> bool {
		*self == MobileWebPlatformIri || *self == MOBILE_WEB_PLATFORM_LABEL
	}
}
