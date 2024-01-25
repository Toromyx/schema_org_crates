/// <https://schema.org/estimatesRiskOf>
pub const ESTIMATES_RISK_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/estimatesRiskOf";
/// <https://schema.org/estimatesRiskOf>
pub const ESTIMATES_RISK_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/estimatesRiskOf";
/// <https://schema.org/estimatesRiskOf>
pub const ESTIMATES_RISK_OF_PROPERTY_LABEL: &str = "estimatesRiskOf";
pub struct EstimatesRiskOfPropertyIri;
impl PartialEq<&str> for EstimatesRiskOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ESTIMATES_RISK_OF_PROPERTY_IRI_HTTP
			|| *other == ESTIMATES_RISK_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EstimatesRiskOfPropertyIri> for &str {
	fn eq(&self, other: &EstimatesRiskOfPropertyIri) -> bool {
		*self == ESTIMATES_RISK_OF_PROPERTY_IRI_HTTP
			|| *self == ESTIMATES_RISK_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct EstimatesRiskOfPropertyIriOrLabel;
impl PartialEq<&str> for EstimatesRiskOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EstimatesRiskOfPropertyIri || *other == ESTIMATES_RISK_OF_PROPERTY_LABEL
	}
}
impl PartialEq<EstimatesRiskOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EstimatesRiskOfPropertyIriOrLabel) -> bool {
		*self == EstimatesRiskOfPropertyIri || *self == ESTIMATES_RISK_OF_PROPERTY_LABEL
	}
}
