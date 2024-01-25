/// <https://schema.org/NoninvasiveProcedure>
pub const NONINVASIVE_PROCEDURE_IRI_HTTP: &str = "http://schema.org/NoninvasiveProcedure";
/// <https://schema.org/NoninvasiveProcedure>
pub const NONINVASIVE_PROCEDURE_IRI_HTTPS: &str = "https://schema.org/NoninvasiveProcedure";
/// <https://schema.org/NoninvasiveProcedure>
pub const NONINVASIVE_PROCEDURE_LABEL: &str = "NoninvasiveProcedure";
pub struct NoninvasiveProcedureIri;
impl PartialEq<&str> for NoninvasiveProcedureIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONINVASIVE_PROCEDURE_IRI_HTTP || *other == NONINVASIVE_PROCEDURE_IRI_HTTPS
	}
}
impl PartialEq<NoninvasiveProcedureIri> for &str {
	fn eq(&self, other: &NoninvasiveProcedureIri) -> bool {
		*self == NONINVASIVE_PROCEDURE_IRI_HTTP || *self == NONINVASIVE_PROCEDURE_IRI_HTTPS
	}
}
pub struct NoninvasiveProcedureIriOrLabel;
impl PartialEq<&str> for NoninvasiveProcedureIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NoninvasiveProcedureIri || *other == NONINVASIVE_PROCEDURE_LABEL
	}
}
impl PartialEq<NoninvasiveProcedureIriOrLabel> for &str {
	fn eq(&self, other: &NoninvasiveProcedureIriOrLabel) -> bool {
		*self == NoninvasiveProcedureIri || *self == NONINVASIVE_PROCEDURE_LABEL
	}
}
