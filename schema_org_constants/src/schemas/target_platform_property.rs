/// <https://schema.org/targetPlatform>
pub const TARGET_PLATFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/targetPlatform";
/// <https://schema.org/targetPlatform>
pub const TARGET_PLATFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/targetPlatform";
/// <https://schema.org/targetPlatform>
pub const TARGET_PLATFORM_PROPERTY_LABEL: &str = "targetPlatform";
pub struct TargetPlatformPropertyIri;
impl PartialEq<&str> for TargetPlatformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TARGET_PLATFORM_PROPERTY_IRI_HTTP || *other == TARGET_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TargetPlatformPropertyIri> for &str {
	fn eq(&self, other: &TargetPlatformPropertyIri) -> bool {
		*self == TARGET_PLATFORM_PROPERTY_IRI_HTTP || *self == TARGET_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct TargetPlatformPropertyIriOrLabel;
impl PartialEq<&str> for TargetPlatformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TargetPlatformPropertyIri || *other == TARGET_PLATFORM_PROPERTY_LABEL
	}
}
impl PartialEq<TargetPlatformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TargetPlatformPropertyIriOrLabel) -> bool {
		*self == TargetPlatformPropertyIri || *self == TARGET_PLATFORM_PROPERTY_LABEL
	}
}
