/// <https://schema.org/accessMode>
pub const ACCESS_MODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessMode";
/// <https://schema.org/accessMode>
pub const ACCESS_MODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accessMode";
/// <https://schema.org/accessMode>
pub const ACCESS_MODE_PROPERTY_LABEL: &str = "accessMode";
pub struct AccessModePropertyIri;
impl PartialEq<&str> for AccessModePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESS_MODE_PROPERTY_IRI_HTTP || *other == ACCESS_MODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessModePropertyIri> for &str {
	fn eq(&self, other: &AccessModePropertyIri) -> bool {
		*self == ACCESS_MODE_PROPERTY_IRI_HTTP || *self == ACCESS_MODE_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessModePropertyIriOrLabel;
impl PartialEq<&str> for AccessModePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessModePropertyIri || *other == ACCESS_MODE_PROPERTY_LABEL
	}
}
impl PartialEq<AccessModePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessModePropertyIriOrLabel) -> bool {
		*self == AccessModePropertyIri || *self == ACCESS_MODE_PROPERTY_LABEL
	}
}
