/// <https://schema.org/RealEstateAgent>
pub const REAL_ESTATE_AGENT_IRI_HTTP: &str = "http://schema.org/RealEstateAgent";
/// <https://schema.org/RealEstateAgent>
pub const REAL_ESTATE_AGENT_IRI_HTTPS: &str = "https://schema.org/RealEstateAgent";
/// <https://schema.org/RealEstateAgent>
pub const REAL_ESTATE_AGENT_LABEL: &str = "RealEstateAgent";
pub struct RealEstateAgentIri;
impl PartialEq<&str> for RealEstateAgentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REAL_ESTATE_AGENT_IRI_HTTP || *other == REAL_ESTATE_AGENT_IRI_HTTPS
	}
}
impl PartialEq<RealEstateAgentIri> for &str {
	fn eq(&self, other: &RealEstateAgentIri) -> bool {
		*self == REAL_ESTATE_AGENT_IRI_HTTP || *self == REAL_ESTATE_AGENT_IRI_HTTPS
	}
}
pub struct RealEstateAgentIriOrLabel;
impl PartialEq<&str> for RealEstateAgentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RealEstateAgentIri || *other == REAL_ESTATE_AGENT_LABEL
	}
}
impl PartialEq<RealEstateAgentIriOrLabel> for &str {
	fn eq(&self, other: &RealEstateAgentIriOrLabel) -> bool {
		*self == RealEstateAgentIri || *self == REAL_ESTATE_AGENT_LABEL
	}
}
