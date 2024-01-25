/// <https://schema.org/actionPlatform>
pub const ACTION_PLATFORM_PROPERTY_IRI_HTTP: &str = "http://schema.org/actionPlatform";
/// <https://schema.org/actionPlatform>
pub const ACTION_PLATFORM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/actionPlatform";
/// <https://schema.org/actionPlatform>
pub const ACTION_PLATFORM_PROPERTY_LABEL: &str = "actionPlatform";
pub struct ActionPlatformPropertyIri;
impl PartialEq<&str> for ActionPlatformPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTION_PLATFORM_PROPERTY_IRI_HTTP || *other == ACTION_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActionPlatformPropertyIri> for &str {
	fn eq(&self, other: &ActionPlatformPropertyIri) -> bool {
		*self == ACTION_PLATFORM_PROPERTY_IRI_HTTP || *self == ACTION_PLATFORM_PROPERTY_IRI_HTTPS
	}
}
pub struct ActionPlatformPropertyIriOrLabel;
impl PartialEq<&str> for ActionPlatformPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActionPlatformPropertyIri || *other == ACTION_PLATFORM_PROPERTY_LABEL
	}
}
impl PartialEq<ActionPlatformPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActionPlatformPropertyIriOrLabel) -> bool {
		*self == ActionPlatformPropertyIri || *self == ACTION_PLATFORM_PROPERTY_LABEL
	}
}
