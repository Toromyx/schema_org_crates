/// <https://schema.org/infectiousAgentClass>
pub const INFECTIOUS_AGENT_CLASS_PROPERTY_IRI_HTTP: &str = "http://schema.org/infectiousAgentClass";
/// <https://schema.org/infectiousAgentClass>
pub const INFECTIOUS_AGENT_CLASS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/infectiousAgentClass";
/// <https://schema.org/infectiousAgentClass>
pub const INFECTIOUS_AGENT_CLASS_PROPERTY_LABEL: &str = "infectiousAgentClass";
pub struct InfectiousAgentClassPropertyIri;
impl PartialEq<&str> for InfectiousAgentClassPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INFECTIOUS_AGENT_CLASS_PROPERTY_IRI_HTTP
			|| *other == INFECTIOUS_AGENT_CLASS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InfectiousAgentClassPropertyIri> for &str {
	fn eq(&self, other: &InfectiousAgentClassPropertyIri) -> bool {
		*self == INFECTIOUS_AGENT_CLASS_PROPERTY_IRI_HTTP
			|| *self == INFECTIOUS_AGENT_CLASS_PROPERTY_IRI_HTTPS
	}
}
pub struct InfectiousAgentClassPropertyIriOrLabel;
impl PartialEq<&str> for InfectiousAgentClassPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InfectiousAgentClassPropertyIri || *other == INFECTIOUS_AGENT_CLASS_PROPERTY_LABEL
	}
}
impl PartialEq<InfectiousAgentClassPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InfectiousAgentClassPropertyIriOrLabel) -> bool {
		*self == InfectiousAgentClassPropertyIri || *self == INFECTIOUS_AGENT_CLASS_PROPERTY_LABEL
	}
}
