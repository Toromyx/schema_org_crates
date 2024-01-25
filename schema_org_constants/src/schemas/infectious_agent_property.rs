/// <https://schema.org/infectiousAgent>
pub const INFECTIOUS_AGENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/infectiousAgent";
/// <https://schema.org/infectiousAgent>
pub const INFECTIOUS_AGENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/infectiousAgent";
/// <https://schema.org/infectiousAgent>
pub const INFECTIOUS_AGENT_PROPERTY_LABEL: &str = "infectiousAgent";
pub struct InfectiousAgentPropertyIri;
impl PartialEq<&str> for InfectiousAgentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INFECTIOUS_AGENT_PROPERTY_IRI_HTTP
			|| *other == INFECTIOUS_AGENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InfectiousAgentPropertyIri> for &str {
	fn eq(&self, other: &InfectiousAgentPropertyIri) -> bool {
		*self == INFECTIOUS_AGENT_PROPERTY_IRI_HTTP || *self == INFECTIOUS_AGENT_PROPERTY_IRI_HTTPS
	}
}
pub struct InfectiousAgentPropertyIriOrLabel;
impl PartialEq<&str> for InfectiousAgentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InfectiousAgentPropertyIri || *other == INFECTIOUS_AGENT_PROPERTY_LABEL
	}
}
impl PartialEq<InfectiousAgentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InfectiousAgentPropertyIriOrLabel) -> bool {
		*self == InfectiousAgentPropertyIri || *self == INFECTIOUS_AGENT_PROPERTY_LABEL
	}
}
