/// <https://schema.org/MedicalGuidelineContraindication>
pub const MEDICAL_GUIDELINE_CONTRAINDICATION_IRI_HTTP: &str =
	"http://schema.org/MedicalGuidelineContraindication";
/// <https://schema.org/MedicalGuidelineContraindication>
pub const MEDICAL_GUIDELINE_CONTRAINDICATION_IRI_HTTPS: &str =
	"https://schema.org/MedicalGuidelineContraindication";
/// <https://schema.org/MedicalGuidelineContraindication>
pub const MEDICAL_GUIDELINE_CONTRAINDICATION_LABEL: &str = "MedicalGuidelineContraindication";
pub struct MedicalGuidelineContraindicationIri;
impl PartialEq<&str> for MedicalGuidelineContraindicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_GUIDELINE_CONTRAINDICATION_IRI_HTTP
			|| *other == MEDICAL_GUIDELINE_CONTRAINDICATION_IRI_HTTPS
	}
}
impl PartialEq<MedicalGuidelineContraindicationIri> for &str {
	fn eq(&self, other: &MedicalGuidelineContraindicationIri) -> bool {
		*self == MEDICAL_GUIDELINE_CONTRAINDICATION_IRI_HTTP
			|| *self == MEDICAL_GUIDELINE_CONTRAINDICATION_IRI_HTTPS
	}
}
pub struct MedicalGuidelineContraindicationIriOrLabel;
impl PartialEq<&str> for MedicalGuidelineContraindicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalGuidelineContraindicationIri
			|| *other == MEDICAL_GUIDELINE_CONTRAINDICATION_LABEL
	}
}
impl PartialEq<MedicalGuidelineContraindicationIriOrLabel> for &str {
	fn eq(&self, other: &MedicalGuidelineContraindicationIriOrLabel) -> bool {
		*self == MedicalGuidelineContraindicationIri
			|| *self == MEDICAL_GUIDELINE_CONTRAINDICATION_LABEL
	}
}
