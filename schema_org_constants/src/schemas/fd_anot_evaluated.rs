/// <https://schema.org/FDAnotEvaluated>
pub const FD_ANOT_EVALUATED_IRI_HTTP: &str = "http://schema.org/FDAnotEvaluated";
/// <https://schema.org/FDAnotEvaluated>
pub const FD_ANOT_EVALUATED_IRI_HTTPS: &str = "https://schema.org/FDAnotEvaluated";
/// <https://schema.org/FDAnotEvaluated>
pub const FD_ANOT_EVALUATED_LABEL: &str = "FDAnotEvaluated";
pub struct FdAnotEvaluatedIri;
impl PartialEq<&str> for FdAnotEvaluatedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FD_ANOT_EVALUATED_IRI_HTTP || *other == FD_ANOT_EVALUATED_IRI_HTTPS
	}
}
impl PartialEq<FdAnotEvaluatedIri> for &str {
	fn eq(&self, other: &FdAnotEvaluatedIri) -> bool {
		*self == FD_ANOT_EVALUATED_IRI_HTTP || *self == FD_ANOT_EVALUATED_IRI_HTTPS
	}
}
pub struct FdAnotEvaluatedIriOrLabel;
impl PartialEq<&str> for FdAnotEvaluatedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FdAnotEvaluatedIri || *other == FD_ANOT_EVALUATED_LABEL
	}
}
impl PartialEq<FdAnotEvaluatedIriOrLabel> for &str {
	fn eq(&self, other: &FdAnotEvaluatedIriOrLabel) -> bool {
		*self == FdAnotEvaluatedIri || *self == FD_ANOT_EVALUATED_LABEL
	}
}
