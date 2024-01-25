/// <https://schema.org/riskFactor>
pub const RISK_FACTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/riskFactor";
/// <https://schema.org/riskFactor>
pub const RISK_FACTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/riskFactor";
/// <https://schema.org/riskFactor>
pub const RISK_FACTOR_PROPERTY_LABEL: &str = "riskFactor";
pub struct RiskFactorPropertyIri;
impl PartialEq<&str> for RiskFactorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RISK_FACTOR_PROPERTY_IRI_HTTP || *other == RISK_FACTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RiskFactorPropertyIri> for &str {
	fn eq(&self, other: &RiskFactorPropertyIri) -> bool {
		*self == RISK_FACTOR_PROPERTY_IRI_HTTP || *self == RISK_FACTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct RiskFactorPropertyIriOrLabel;
impl PartialEq<&str> for RiskFactorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RiskFactorPropertyIri || *other == RISK_FACTOR_PROPERTY_LABEL
	}
}
impl PartialEq<RiskFactorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RiskFactorPropertyIriOrLabel) -> bool {
		*self == RiskFactorPropertyIri || *self == RISK_FACTOR_PROPERTY_LABEL
	}
}
