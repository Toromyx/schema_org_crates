/// <https://schema.org/PalliativeProcedure>
pub const PALLIATIVE_PROCEDURE_IRI_HTTP: &str = "http://schema.org/PalliativeProcedure";
/// <https://schema.org/PalliativeProcedure>
pub const PALLIATIVE_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/PalliativeProcedure";
/// <https://schema.org/PalliativeProcedure>
pub const PALLIATIVE_PROCEDURE_LABEL: &str = "PalliativeProcedure";
pub struct PalliativeProcedureIri;
impl PartialEq<&str> for PalliativeProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PALLIATIVE_PROCEDURE_IRI_HTTP || *other == PALLIATIVE_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<PalliativeProcedureIri> for &str {
	fn eq(&self, other: &PalliativeProcedureIri) -> bool {
		*self == PALLIATIVE_PROCEDURE_IRI_HTTP || *self == PALLIATIVE_PROCEDURE_IRI_HTTPS
	}
}
pub struct PalliativeProcedureIriOrLabel;
impl PartialEq<&str> for PalliativeProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PalliativeProcedureIri || *other == PALLIATIVE_PROCEDURE_LABEL
	}
}
impl PartialEq<PalliativeProcedureIriOrLabel> for &str {
	fn eq(&self, other: &PalliativeProcedureIriOrLabel) -> bool {
		*self == PalliativeProcedureIri || *self == PALLIATIVE_PROCEDURE_LABEL
	}
}
