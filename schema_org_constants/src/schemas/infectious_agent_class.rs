/// <https://schema.org/InfectiousAgentClass>
pub const INFECTIOUS_AGENT_CLASS_IRI_HTTP: &str = "http://schema.org/InfectiousAgentClass";
/// <https://schema.org/InfectiousAgentClass>
pub const INFECTIOUS_AGENT_CLASS_IRI_HTTPS: &str = "https://schema.org/InfectiousAgentClass";
/// <https://schema.org/InfectiousAgentClass>
pub const INFECTIOUS_AGENT_CLASS_LABEL: &str = "InfectiousAgentClass";
pub struct InfectiousAgentClassIri;
impl PartialEq<&str> for InfectiousAgentClassIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INFECTIOUS_AGENT_CLASS_IRI_HTTP || *other == INFECTIOUS_AGENT_CLASS_IRI_HTTPS
	}
}
impl PartialEq<InfectiousAgentClassIri> for &str {
	fn eq(&self, other: &InfectiousAgentClassIri) -> bool {
		*self == INFECTIOUS_AGENT_CLASS_IRI_HTTP || *self == INFECTIOUS_AGENT_CLASS_IRI_HTTPS
	}
}
pub struct InfectiousAgentClassIriOrLabel;
impl PartialEq<&str> for InfectiousAgentClassIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InfectiousAgentClassIri || *other == INFECTIOUS_AGENT_CLASS_LABEL
	}
}
impl PartialEq<InfectiousAgentClassIriOrLabel> for &str {
	fn eq(&self, other: &InfectiousAgentClassIriOrLabel) -> bool {
		*self == InfectiousAgentClassIri || *self == INFECTIOUS_AGENT_CLASS_LABEL
	}
}
