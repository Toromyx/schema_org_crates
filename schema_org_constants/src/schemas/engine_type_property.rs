/// <https://schema.org/engineType>
pub const ENGINE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/engineType";
/// <https://schema.org/engineType>
pub const ENGINE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/engineType";
/// <https://schema.org/engineType>
pub const ENGINE_TYPE_PROPERTY_LABEL: &str = "engineType";
pub struct EngineTypePropertyIri;
impl PartialEq<&str> for EngineTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENGINE_TYPE_PROPERTY_IRI_HTTP || *other == ENGINE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EngineTypePropertyIri> for &str {
	fn eq(&self, other: &EngineTypePropertyIri) -> bool {
		*self == ENGINE_TYPE_PROPERTY_IRI_HTTP || *self == ENGINE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct EngineTypePropertyIriOrLabel;
impl PartialEq<&str> for EngineTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EngineTypePropertyIri || *other == ENGINE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<EngineTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EngineTypePropertyIriOrLabel) -> bool {
		*self == EngineTypePropertyIri || *self == ENGINE_TYPE_PROPERTY_LABEL
	}
}
