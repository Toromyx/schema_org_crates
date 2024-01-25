/// <https://schema.org/CoOp>
pub const CO_OP_IRI_HTTP: &str = "http://schema.org/CoOp";
/// <https://schema.org/CoOp>
pub const CO_OP_IRI_HTTPS: &str = "https://schema.org/CoOp";
/// <https://schema.org/CoOp>
pub const CO_OP_LABEL: &str = "CoOp";
pub struct CoOpIri;
impl PartialEq<&str> for CoOpIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CO_OP_IRI_HTTP || *other == CO_OP_IRI_HTTPS
	}
}
impl PartialEq<CoOpIri> for &str {
	fn eq(&self, other: &CoOpIri) -> bool {
		*self == CO_OP_IRI_HTTP || *self == CO_OP_IRI_HTTPS
	}
}
pub struct CoOpIriOrLabel;
impl PartialEq<&str> for CoOpIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CoOpIri || *other == CO_OP_LABEL
	}
}
impl PartialEq<CoOpIriOrLabel> for &str {
	fn eq(&self, other: &CoOpIriOrLabel) -> bool {
		*self == CoOpIri || *self == CO_OP_LABEL
	}
}
