/// <https://schema.org/MedicalRiskEstimator>
pub const MEDICAL_RISK_ESTIMATOR_IRI_HTTP: &str = "http://schema.org/MedicalRiskEstimator";
/// <https://schema.org/MedicalRiskEstimator>
pub const MEDICAL_RISK_ESTIMATOR_IRI_HTTPS: &str = "https://schema.org/MedicalRiskEstimator";
/// <https://schema.org/MedicalRiskEstimator>
pub const MEDICAL_RISK_ESTIMATOR_LABEL: &str = "MedicalRiskEstimator";
pub struct MedicalRiskEstimatorIri;
impl PartialEq<&str> for MedicalRiskEstimatorIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_RISK_ESTIMATOR_IRI_HTTP || *other == MEDICAL_RISK_ESTIMATOR_IRI_HTTPS
	}
}
impl PartialEq<MedicalRiskEstimatorIri> for &str {
	fn eq(&self, other: &MedicalRiskEstimatorIri) -> bool {
		*self == MEDICAL_RISK_ESTIMATOR_IRI_HTTP || *self == MEDICAL_RISK_ESTIMATOR_IRI_HTTPS
	}
}
pub struct MedicalRiskEstimatorIriOrLabel;
impl PartialEq<&str> for MedicalRiskEstimatorIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalRiskEstimatorIri || *other == MEDICAL_RISK_ESTIMATOR_LABEL
	}
}
impl PartialEq<MedicalRiskEstimatorIriOrLabel> for &str {
	fn eq(&self, other: &MedicalRiskEstimatorIriOrLabel) -> bool {
		*self == MedicalRiskEstimatorIri || *self == MEDICAL_RISK_ESTIMATOR_LABEL
	}
}
