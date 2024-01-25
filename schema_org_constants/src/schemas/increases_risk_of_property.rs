/// <https://schema.org/increasesRiskOf>
pub const INCREASES_RISK_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/increasesRiskOf";
/// <https://schema.org/increasesRiskOf>
pub const INCREASES_RISK_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/increasesRiskOf";
/// <https://schema.org/increasesRiskOf>
pub const INCREASES_RISK_OF_PROPERTY_LABEL: &str = "increasesRiskOf";
pub struct IncreasesRiskOfPropertyIri;
impl PartialEq<&str> for IncreasesRiskOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCREASES_RISK_OF_PROPERTY_IRI_HTTP
			|| *other == INCREASES_RISK_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncreasesRiskOfPropertyIri> for &str {
	fn eq(&self, other: &IncreasesRiskOfPropertyIri) -> bool {
		*self == INCREASES_RISK_OF_PROPERTY_IRI_HTTP
			|| *self == INCREASES_RISK_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct IncreasesRiskOfPropertyIriOrLabel;
impl PartialEq<&str> for IncreasesRiskOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncreasesRiskOfPropertyIri || *other == INCREASES_RISK_OF_PROPERTY_LABEL
	}
}
impl PartialEq<IncreasesRiskOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncreasesRiskOfPropertyIriOrLabel) -> bool {
		*self == IncreasesRiskOfPropertyIri || *self == INCREASES_RISK_OF_PROPERTY_LABEL
	}
}
