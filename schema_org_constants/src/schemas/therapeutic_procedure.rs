/// <https://schema.org/TherapeuticProcedure>
pub const THERAPEUTIC_PROCEDURE_IRI_HTTP: &str = "http://schema.org/TherapeuticProcedure";
/// <https://schema.org/TherapeuticProcedure>
pub const THERAPEUTIC_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/TherapeuticProcedure";
/// <https://schema.org/TherapeuticProcedure>
pub const THERAPEUTIC_PROCEDURE_LABEL: &str = "TherapeuticProcedure";
pub struct TherapeuticProcedureIri;
impl PartialEq<&str> for TherapeuticProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THERAPEUTIC_PROCEDURE_IRI_HTTP || *other == THERAPEUTIC_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<TherapeuticProcedureIri> for &str {
	fn eq(&self, other: &TherapeuticProcedureIri) -> bool {
		*self == THERAPEUTIC_PROCEDURE_IRI_HTTP || *self == THERAPEUTIC_PROCEDURE_IRI_HTTPS
	}
}
pub struct TherapeuticProcedureIriOrLabel;
impl PartialEq<&str> for TherapeuticProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TherapeuticProcedureIri || *other == THERAPEUTIC_PROCEDURE_LABEL
	}
}
impl PartialEq<TherapeuticProcedureIriOrLabel> for &str {
	fn eq(&self, other: &TherapeuticProcedureIriOrLabel) -> bool {
		*self == TherapeuticProcedureIri || *self == THERAPEUTIC_PROCEDURE_LABEL
	}
}
