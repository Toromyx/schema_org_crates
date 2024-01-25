/// <https://schema.org/Answer>
pub const ANSWER_IRI_HTTP: &str = "http://schema.org/Answer";
/// <https://schema.org/Answer>
pub const ANSWER_IRI_HTTPS: &str = "https://schema.org/Answer";
/// <https://schema.org/Answer>
pub const ANSWER_LABEL: &str = "Answer";
pub struct AnswerIri;
impl PartialEq<&str> for AnswerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANSWER_IRI_HTTP || *other == ANSWER_IRI_HTTPS
	}
}
impl PartialEq<AnswerIri> for &str {
	fn eq(&self, other: &AnswerIri) -> bool {
		*self == ANSWER_IRI_HTTP || *self == ANSWER_IRI_HTTPS
	}
}
pub struct AnswerIriOrLabel;
impl PartialEq<&str> for AnswerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnswerIri || *other == ANSWER_LABEL
	}
}
impl PartialEq<AnswerIriOrLabel> for &str {
	fn eq(&self, other: &AnswerIriOrLabel) -> bool {
		*self == AnswerIri || *self == ANSWER_LABEL
	}
}
