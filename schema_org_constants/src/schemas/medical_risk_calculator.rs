/// <https://schema.org/MedicalRiskCalculator>
pub const MEDICAL_RISK_CALCULATOR_IRI_HTTP: &str = "http://schema.org/MedicalRiskCalculator";
/// <https://schema.org/MedicalRiskCalculator>
pub const MEDICAL_RISK_CALCULATOR_IRI_HTTPS: &str = "https://schema.org/MedicalRiskCalculator";
/// <https://schema.org/MedicalRiskCalculator>
pub const MEDICAL_RISK_CALCULATOR_LABEL: &str = "MedicalRiskCalculator";
pub struct MedicalRiskCalculatorIri;
impl PartialEq<&str> for MedicalRiskCalculatorIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_RISK_CALCULATOR_IRI_HTTP || *other == MEDICAL_RISK_CALCULATOR_IRI_HTTPS
	}
}
impl PartialEq<MedicalRiskCalculatorIri> for &str {
	fn eq(&self, other: &MedicalRiskCalculatorIri) -> bool {
		*self == MEDICAL_RISK_CALCULATOR_IRI_HTTP || *self == MEDICAL_RISK_CALCULATOR_IRI_HTTPS
	}
}
pub struct MedicalRiskCalculatorIriOrLabel;
impl PartialEq<&str> for MedicalRiskCalculatorIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalRiskCalculatorIri || *other == MEDICAL_RISK_CALCULATOR_LABEL
	}
}
impl PartialEq<MedicalRiskCalculatorIriOrLabel> for &str {
	fn eq(&self, other: &MedicalRiskCalculatorIriOrLabel) -> bool {
		*self == MedicalRiskCalculatorIri || *self == MEDICAL_RISK_CALCULATOR_LABEL
	}
}
