/// <https://schema.org/DesktopWebPlatform>
pub const DESKTOP_WEB_PLATFORM_IRI_HTTP: &str = "http://schema.org/DesktopWebPlatform";
/// <https://schema.org/DesktopWebPlatform>
pub const DESKTOP_WEB_PLATFORM_IRI_HTTPS: &str = "https://schema.org/DesktopWebPlatform";
/// <https://schema.org/DesktopWebPlatform>
pub const DESKTOP_WEB_PLATFORM_LABEL: &str = "DesktopWebPlatform";
pub struct DesktopWebPlatformIri;
impl PartialEq<&str> for DesktopWebPlatformIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DESKTOP_WEB_PLATFORM_IRI_HTTP || *other == DESKTOP_WEB_PLATFORM_IRI_HTTPS
	}
}
impl PartialEq<DesktopWebPlatformIri> for &str {
	fn eq(&self, other: &DesktopWebPlatformIri) -> bool {
		*self == DESKTOP_WEB_PLATFORM_IRI_HTTP || *self == DESKTOP_WEB_PLATFORM_IRI_HTTPS
	}
}
pub struct DesktopWebPlatformIriOrLabel;
impl PartialEq<&str> for DesktopWebPlatformIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DesktopWebPlatformIri || *other == DESKTOP_WEB_PLATFORM_LABEL
	}
}
impl PartialEq<DesktopWebPlatformIriOrLabel> for &str {
	fn eq(&self, other: &DesktopWebPlatformIriOrLabel) -> bool {
		*self == DesktopWebPlatformIri || *self == DESKTOP_WEB_PLATFORM_LABEL
	}
}
