/// <https://schema.org/MedicalContraindication>
pub const MEDICAL_CONTRAINDICATION_IRI_HTTP: &str = "http://schema.org/MedicalContraindication";
/// <https://schema.org/MedicalContraindication>
pub const MEDICAL_CONTRAINDICATION_IRI_HTTPS: &str = "https://schema.org/MedicalContraindication";
/// <https://schema.org/MedicalContraindication>
pub const MEDICAL_CONTRAINDICATION_LABEL: &str = "MedicalContraindication";
pub struct MedicalContraindicationIri;
impl PartialEq<&str> for MedicalContraindicationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_CONTRAINDICATION_IRI_HTTP || *other == MEDICAL_CONTRAINDICATION_IRI_HTTPS
	}
}
impl PartialEq<MedicalContraindicationIri> for &str {
	fn eq(&self, other: &MedicalContraindicationIri) -> bool {
		*self == MEDICAL_CONTRAINDICATION_IRI_HTTP || *self == MEDICAL_CONTRAINDICATION_IRI_HTTPS
	}
}
pub struct MedicalContraindicationIriOrLabel;
impl PartialEq<&str> for MedicalContraindicationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalContraindicationIri || *other == MEDICAL_CONTRAINDICATION_LABEL
	}
}
impl PartialEq<MedicalContraindicationIriOrLabel> for &str {
	fn eq(&self, other: &MedicalContraindicationIriOrLabel) -> bool {
		*self == MedicalContraindicationIri || *self == MEDICAL_CONTRAINDICATION_LABEL
	}
}
