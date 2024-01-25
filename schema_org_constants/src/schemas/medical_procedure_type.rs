/// <https://schema.org/MedicalProcedureType>
pub const MEDICAL_PROCEDURE_TYPE_IRI_HTTP: &str = "http://schema.org/MedicalProcedureType";
/// <https://schema.org/MedicalProcedureType>
pub const MEDICAL_PROCEDURE_TYPE_IRI_HTTPS: &str = "https://schema.org/MedicalProcedureType";
/// <https://schema.org/MedicalProcedureType>
pub const MEDICAL_PROCEDURE_TYPE_LABEL: &str = "MedicalProcedureType";
pub struct MedicalProcedureTypeIri;
impl PartialEq<&str> for MedicalProcedureTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_PROCEDURE_TYPE_IRI_HTTP || *other == MEDICAL_PROCEDURE_TYPE_IRI_HTTPS
	}
}
impl PartialEq<MedicalProcedureTypeIri> for &str {
	fn eq(&self, other: &MedicalProcedureTypeIri) -> bool {
		*self == MEDICAL_PROCEDURE_TYPE_IRI_HTTP || *self == MEDICAL_PROCEDURE_TYPE_IRI_HTTPS
	}
}
pub struct MedicalProcedureTypeIriOrLabel;
impl PartialEq<&str> for MedicalProcedureTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalProcedureTypeIri || *other == MEDICAL_PROCEDURE_TYPE_LABEL
	}
}
impl PartialEq<MedicalProcedureTypeIriOrLabel> for &str {
	fn eq(&self, other: &MedicalProcedureTypeIriOrLabel) -> bool {
		*self == MedicalProcedureTypeIri || *self == MEDICAL_PROCEDURE_TYPE_LABEL
	}
}
