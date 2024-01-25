/// <https://schema.org/MedicalProcedure>
pub const MEDICAL_PROCEDURE_IRI_HTTP: &str = "http://schema.org/MedicalProcedure";
/// <https://schema.org/MedicalProcedure>
pub const MEDICAL_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/MedicalProcedure";
/// <https://schema.org/MedicalProcedure>
pub const MEDICAL_PROCEDURE_LABEL: &str = "MedicalProcedure";
pub struct MedicalProcedureIri;
impl PartialEq<&str> for MedicalProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICAL_PROCEDURE_IRI_HTTP || *other == MEDICAL_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<MedicalProcedureIri> for &str {
	fn eq(&self, other: &MedicalProcedureIri) -> bool {
		*self == MEDICAL_PROCEDURE_IRI_HTTP || *self == MEDICAL_PROCEDURE_IRI_HTTPS
	}
}
pub struct MedicalProcedureIriOrLabel;
impl PartialEq<&str> for MedicalProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicalProcedureIri || *other == MEDICAL_PROCEDURE_LABEL
	}
}
impl PartialEq<MedicalProcedureIriOrLabel> for &str {
	fn eq(&self, other: &MedicalProcedureIriOrLabel) -> bool {
		*self == MedicalProcedureIri || *self == MEDICAL_PROCEDURE_LABEL
	}
}
