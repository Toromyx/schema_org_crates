/// <https://schema.org/MedicalRiskScore>
pub const MEDICAL_RISK_SCORE_IRI_HTTP: &str = "http://schema.org/MedicalRiskScore";
/// <https://schema.org/MedicalRiskScore>
pub const MEDICAL_RISK_SCORE_IRI_HTTPS: &str = "https://schema.org/MedicalRiskScore";
/// <https://schema.org/MedicalRiskScore>
pub const MEDICAL_RISK_SCORE_LABEL: &str = "MedicalRiskScore";
pub struct MedicalRiskScoreIri;
impl PartialEq<&str> for MedicalRiskScoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_RISK_SCORE_IRI_HTTP || *other == MEDICAL_RISK_SCORE_IRI_HTTPS
	}
}
impl PartialEq<MedicalRiskScoreIri> for &str {
	fn eq(&self, other: &MedicalRiskScoreIri) -> bool {
		*self == MEDICAL_RISK_SCORE_IRI_HTTP || *self == MEDICAL_RISK_SCORE_IRI_HTTPS
	}
}
pub struct MedicalRiskScoreIriOrLabel;
impl PartialEq<&str> for MedicalRiskScoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalRiskScoreIri || *other == MEDICAL_RISK_SCORE_LABEL
	}
}
impl PartialEq<MedicalRiskScoreIriOrLabel> for &str {
	fn eq(&self, other: &MedicalRiskScoreIriOrLabel) -> bool {
		*self == MedicalRiskScoreIri || *self == MEDICAL_RISK_SCORE_LABEL
	}
}
