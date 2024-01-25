/// <https://schema.org/MedicalRiskFactor>
pub const MEDICAL_RISK_FACTOR_IRI_HTTP: &str = "http://schema.org/MedicalRiskFactor";
/// <https://schema.org/MedicalRiskFactor>
pub const MEDICAL_RISK_FACTOR_IRI_HTTPS: &str = "https://schema.org/MedicalRiskFactor";
/// <https://schema.org/MedicalRiskFactor>
pub const MEDICAL_RISK_FACTOR_LABEL: &str = "MedicalRiskFactor";
pub struct MedicalRiskFactorIri;
impl PartialEq<&str> for MedicalRiskFactorIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_RISK_FACTOR_IRI_HTTP || *other == MEDICAL_RISK_FACTOR_IRI_HTTPS
	}
}
impl PartialEq<MedicalRiskFactorIri> for &str {
	fn eq(&self, other: &MedicalRiskFactorIri) -> bool {
		*self == MEDICAL_RISK_FACTOR_IRI_HTTP || *self == MEDICAL_RISK_FACTOR_IRI_HTTPS
	}
}
pub struct MedicalRiskFactorIriOrLabel;
impl PartialEq<&str> for MedicalRiskFactorIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalRiskFactorIri || *other == MEDICAL_RISK_FACTOR_LABEL
	}
}
impl PartialEq<MedicalRiskFactorIriOrLabel> for &str {
	fn eq(&self, other: &MedicalRiskFactorIriOrLabel) -> bool {
		*self == MedicalRiskFactorIri || *self == MEDICAL_RISK_FACTOR_LABEL
	}
}
