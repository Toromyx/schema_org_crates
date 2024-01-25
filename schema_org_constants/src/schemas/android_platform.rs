/// <https://schema.org/AndroidPlatform>
pub const ANDROID_PLATFORM_IRI_HTTP: &str = "http://schema.org/AndroidPlatform";
/// <https://schema.org/AndroidPlatform>
pub const ANDROID_PLATFORM_IRI_HTTPS: &str = "https://schema.org/AndroidPlatform";
/// <https://schema.org/AndroidPlatform>
pub const ANDROID_PLATFORM_LABEL: &str = "AndroidPlatform";
pub struct AndroidPlatformIri;
impl PartialEq<&str> for AndroidPlatformIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANDROID_PLATFORM_IRI_HTTP || *other == ANDROID_PLATFORM_IRI_HTTPS
	}
}
impl PartialEq<AndroidPlatformIri> for &str {
	fn eq(&self, other: &AndroidPlatformIri) -> bool {
		*self == ANDROID_PLATFORM_IRI_HTTP || *self == ANDROID_PLATFORM_IRI_HTTPS
	}
}
pub struct AndroidPlatformIriOrLabel;
impl PartialEq<&str> for AndroidPlatformIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AndroidPlatformIri || *other == ANDROID_PLATFORM_LABEL
	}
}
impl PartialEq<AndroidPlatformIriOrLabel> for &str {
	fn eq(&self, other: &AndroidPlatformIriOrLabel) -> bool {
		*self == AndroidPlatformIri || *self == ANDROID_PLATFORM_LABEL
	}
}
