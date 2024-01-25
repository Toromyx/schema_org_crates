/// <https://schema.org/includedRiskFactor>
pub const INCLUDED_RISK_FACTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/includedRiskFactor";
/// <https://schema.org/includedRiskFactor>
pub const INCLUDED_RISK_FACTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/includedRiskFactor";
/// <https://schema.org/includedRiskFactor>
pub const INCLUDED_RISK_FACTOR_PROPERTY_LABEL: &str = "includedRiskFactor";
pub struct IncludedRiskFactorPropertyIri;
impl PartialEq<&str> for IncludedRiskFactorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INCLUDED_RISK_FACTOR_PROPERTY_IRI_HTTP
			|| *other == INCLUDED_RISK_FACTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IncludedRiskFactorPropertyIri> for &str {
	fn eq(&self, other: &IncludedRiskFactorPropertyIri) -> bool {
		*self == INCLUDED_RISK_FACTOR_PROPERTY_IRI_HTTP
			|| *self == INCLUDED_RISK_FACTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct IncludedRiskFactorPropertyIriOrLabel;
impl PartialEq<&str> for IncludedRiskFactorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IncludedRiskFactorPropertyIri || *other == INCLUDED_RISK_FACTOR_PROPERTY_LABEL
	}
}
impl PartialEq<IncludedRiskFactorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IncludedRiskFactorPropertyIriOrLabel) -> bool {
		*self == IncludedRiskFactorPropertyIri || *self == INCLUDED_RISK_FACTOR_PROPERTY_LABEL
	}
}
