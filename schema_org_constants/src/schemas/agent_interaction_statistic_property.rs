/// <https://schema.org/agentInteractionStatistic>
pub const AGENT_INTERACTION_STATISTIC_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/agentInteractionStatistic";
/// <https://schema.org/agentInteractionStatistic>
pub const AGENT_INTERACTION_STATISTIC_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/agentInteractionStatistic";
/// <https://schema.org/agentInteractionStatistic>
pub const AGENT_INTERACTION_STATISTIC_PROPERTY_LABEL: &str = "agentInteractionStatistic";
pub struct AgentInteractionStatisticPropertyIri;
impl PartialEq<&str> for AgentInteractionStatisticPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AGENT_INTERACTION_STATISTIC_PROPERTY_IRI_HTTP
			|| *other == AGENT_INTERACTION_STATISTIC_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AgentInteractionStatisticPropertyIri> for &str {
	fn eq(&self, other: &AgentInteractionStatisticPropertyIri) -> bool {
		*self == AGENT_INTERACTION_STATISTIC_PROPERTY_IRI_HTTP
			|| *self == AGENT_INTERACTION_STATISTIC_PROPERTY_IRI_HTTPS
	}
}
pub struct AgentInteractionStatisticPropertyIriOrLabel;
impl PartialEq<&str> for AgentInteractionStatisticPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AgentInteractionStatisticPropertyIri
			|| *other == AGENT_INTERACTION_STATISTIC_PROPERTY_LABEL
	}
}
impl PartialEq<AgentInteractionStatisticPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AgentInteractionStatisticPropertyIriOrLabel) -> bool {
		*self == AgentInteractionStatisticPropertyIri
			|| *self == AGENT_INTERACTION_STATISTIC_PROPERTY_LABEL
	}
}
