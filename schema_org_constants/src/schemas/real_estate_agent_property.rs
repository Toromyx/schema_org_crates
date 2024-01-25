/// <https://schema.org/realEstateAgent>
pub const REAL_ESTATE_AGENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/realEstateAgent";
/// <https://schema.org/realEstateAgent>
pub const REAL_ESTATE_AGENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/realEstateAgent";
/// <https://schema.org/realEstateAgent>
pub const REAL_ESTATE_AGENT_PROPERTY_LABEL: &str = "realEstateAgent";
pub struct RealEstateAgentPropertyIri;
impl PartialEq<&str> for RealEstateAgentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REAL_ESTATE_AGENT_PROPERTY_IRI_HTTP
			|| *other == REAL_ESTATE_AGENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RealEstateAgentPropertyIri> for &str {
	fn eq(&self, other: &RealEstateAgentPropertyIri) -> bool {
		*self == REAL_ESTATE_AGENT_PROPERTY_IRI_HTTP
			|| *self == REAL_ESTATE_AGENT_PROPERTY_IRI_HTTPS
	}
}
pub struct RealEstateAgentPropertyIriOrLabel;
impl PartialEq<&str> for RealEstateAgentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RealEstateAgentPropertyIri || *other == REAL_ESTATE_AGENT_PROPERTY_LABEL
	}
}
impl PartialEq<RealEstateAgentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RealEstateAgentPropertyIriOrLabel) -> bool {
		*self == RealEstateAgentPropertyIri || *self == REAL_ESTATE_AGENT_PROPERTY_LABEL
	}
}
