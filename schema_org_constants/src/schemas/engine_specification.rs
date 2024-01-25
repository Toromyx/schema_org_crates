/// <https://schema.org/EngineSpecification>
pub const ENGINE_SPECIFICATION_IRI_HTTP: &str = "http://schema.org/EngineSpecification";
/// <https://schema.org/EngineSpecification>
pub const ENGINE_SPECIFICATION_IRI_HTTPS: &str = "https://schema.org/EngineSpecification";
/// <https://schema.org/EngineSpecification>
pub const ENGINE_SPECIFICATION_LABEL: &str = "EngineSpecification";
pub struct EngineSpecificationIri;
impl PartialEq<&str> for EngineSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENGINE_SPECIFICATION_IRI_HTTP || *other == ENGINE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<EngineSpecificationIri> for &str {
	fn eq(&self, other: &EngineSpecificationIri) -> bool {
		*self == ENGINE_SPECIFICATION_IRI_HTTP || *self == ENGINE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct EngineSpecificationIriOrLabel;
impl PartialEq<&str> for EngineSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EngineSpecificationIri || *other == ENGINE_SPECIFICATION_LABEL
	}
}
impl PartialEq<EngineSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &EngineSpecificationIriOrLabel) -> bool {
		*self == EngineSpecificationIri || *self == ENGINE_SPECIFICATION_LABEL
	}
}
