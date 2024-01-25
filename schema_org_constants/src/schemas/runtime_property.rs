/// <https://schema.org/runtime>
#[deprecated = "This schema is superseded by <https://schema.org/runtimePlatform>."]
pub const RUNTIME_PROPERTY_IRI_HTTP: &str = "http://schema.org/runtime";
/// <https://schema.org/runtime>
#[deprecated = "This schema is superseded by <https://schema.org/runtimePlatform>."]
pub const RUNTIME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/runtime";
/// <https://schema.org/runtime>
#[deprecated = "This schema is superseded by <https://schema.org/runtimePlatform>."]
pub const RUNTIME_PROPERTY_LABEL: &str = "runtime";
pub struct RuntimePropertyIri;
impl PartialEq<&str> for RuntimePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RUNTIME_PROPERTY_IRI_HTTP || *other == RUNTIME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RuntimePropertyIri> for &str {
	fn eq(&self, other: &RuntimePropertyIri) -> bool {
		*self == RUNTIME_PROPERTY_IRI_HTTP || *self == RUNTIME_PROPERTY_IRI_HTTPS
	}
}
pub struct RuntimePropertyIriOrLabel;
impl PartialEq<&str> for RuntimePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RuntimePropertyIri || *other == RUNTIME_PROPERTY_LABEL
	}
}
impl PartialEq<RuntimePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RuntimePropertyIriOrLabel) -> bool {
		*self == RuntimePropertyIri || *self == RUNTIME_PROPERTY_LABEL
	}
}
