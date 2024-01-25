/// <https://schema.org/engineDisplacement>
pub const ENGINE_DISPLACEMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/engineDisplacement";
/// <https://schema.org/engineDisplacement>
pub const ENGINE_DISPLACEMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/engineDisplacement";
/// <https://schema.org/engineDisplacement>
pub const ENGINE_DISPLACEMENT_PROPERTY_LABEL: &str = "engineDisplacement";
pub struct EngineDisplacementPropertyIri;
impl PartialEq<&str> for EngineDisplacementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENGINE_DISPLACEMENT_PROPERTY_IRI_HTTP
			|| *other == ENGINE_DISPLACEMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EngineDisplacementPropertyIri> for &str {
	fn eq(&self, other: &EngineDisplacementPropertyIri) -> bool {
		*self == ENGINE_DISPLACEMENT_PROPERTY_IRI_HTTP
			|| *self == ENGINE_DISPLACEMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct EngineDisplacementPropertyIriOrLabel;
impl PartialEq<&str> for EngineDisplacementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EngineDisplacementPropertyIri || *other == ENGINE_DISPLACEMENT_PROPERTY_LABEL
	}
}
impl PartialEq<EngineDisplacementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EngineDisplacementPropertyIriOrLabel) -> bool {
		*self == EngineDisplacementPropertyIri || *self == ENGINE_DISPLACEMENT_PROPERTY_LABEL
	}
}
