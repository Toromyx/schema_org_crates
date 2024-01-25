/// <https://schema.org/agent>
pub const AGENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/agent";
/// <https://schema.org/agent>
pub const AGENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/agent";
/// <https://schema.org/agent>
pub const AGENT_PROPERTY_LABEL: &str = "agent";
pub struct AgentPropertyIri;
impl PartialEq<&str> for AgentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AGENT_PROPERTY_IRI_HTTP || *other == AGENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AgentPropertyIri> for &str {
	fn eq(&self, other: &AgentPropertyIri) -> bool {
		*self == AGENT_PROPERTY_IRI_HTTP || *self == AGENT_PROPERTY_IRI_HTTPS
	}
}
pub struct AgentPropertyIriOrLabel;
impl PartialEq<&str> for AgentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AgentPropertyIri || *other == AGENT_PROPERTY_LABEL
	}
}
impl PartialEq<AgentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AgentPropertyIriOrLabel) -> bool {
		*self == AgentPropertyIri || *self == AGENT_PROPERTY_LABEL
	}
}
