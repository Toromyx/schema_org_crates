/// <https://schema.org/IOSPlatform>
pub const IOS_PLATFORM_IRI_HTTP: &str = "http://schema.org/IOSPlatform";
/// <https://schema.org/IOSPlatform>
pub const IOS_PLATFORM_IRI_HTTPS: &str = "https://schema.org/IOSPlatform";
/// <https://schema.org/IOSPlatform>
pub const IOS_PLATFORM_LABEL: &str = "IOSPlatform";
pub struct IosPlatformIri;
impl PartialEq<&str> for IosPlatformIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IOS_PLATFORM_IRI_HTTP || *other == IOS_PLATFORM_IRI_HTTPS
	}
}
impl PartialEq<IosPlatformIri> for &str {
	fn eq(&self, other: &IosPlatformIri) -> bool {
		*self == IOS_PLATFORM_IRI_HTTP || *self == IOS_PLATFORM_IRI_HTTPS
	}
}
pub struct IosPlatformIriOrLabel;
impl PartialEq<&str> for IosPlatformIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IosPlatformIri || *other == IOS_PLATFORM_LABEL
	}
}
impl PartialEq<IosPlatformIriOrLabel> for &str {
	fn eq(&self, other: &IosPlatformIriOrLabel) -> bool {
		*self == IosPlatformIri || *self == IOS_PLATFORM_LABEL
	}
}
