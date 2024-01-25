/// <https://schema.org/runtimePlatform>
pub const RUNTIME_PLATFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/runtimePlatform";
/// <https://schema.org/runtimePlatform>
pub const RUNTIME_PLATFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/runtimePlatform";
/// <https://schema.org/runtimePlatform>
pub const RUNTIME_PLATFORM_PROPERTY_LABEL: &str = "runtimePlatform";
pub struct RuntimePlatformPropertyIri;
impl PartialEq<&str> for RuntimePlatformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RUNTIME_PLATFORM_PROPERTY_IRI_HTTP
			|| *other == RUNTIME_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RuntimePlatformPropertyIri> for &str {
	fn eq(&self, other: &RuntimePlatformPropertyIri) -> bool {
		*self == RUNTIME_PLATFORM_PROPERTY_IRI_HTTP || *self == RUNTIME_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct RuntimePlatformPropertyIriOrLabel;
impl PartialEq<&str> for RuntimePlatformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RuntimePlatformPropertyIri || *other == RUNTIME_PLATFORM_PROPERTY_LABEL
	}
}
impl PartialEq<RuntimePlatformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RuntimePlatformPropertyIriOrLabel) -> bool {
		*self == RuntimePlatformPropertyIri || *self == RUNTIME_PLATFORM_PROPERTY_LABEL
	}
}
