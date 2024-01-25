/// <https://schema.org/Question>
pub const QUESTION_IRI_HTTP: &str = "http://schema.org/Question";
/// <https://schema.org/Question>
pub const QUESTION_IRI_HTTPS: &str = "https://schema.org/Question";
/// <https://schema.org/Question>
pub const QUESTION_LABEL: &str = "Question";
pub struct QuestionIri;
impl PartialEq<&str> for QuestionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUESTION_IRI_HTTP || *other == QUESTION_IRI_HTTPS
	}
}
impl PartialEq<QuestionIri> for &str {
	fn eq(&self, other: &QuestionIri) -> bool {
		*self == QUESTION_IRI_HTTP || *self == QUESTION_IRI_HTTPS
	}
}
pub struct QuestionIriOrLabel;
impl PartialEq<&str> for QuestionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuestionIri || *other == QUESTION_LABEL
	}
}
impl PartialEq<QuestionIriOrLabel> for &str {
	fn eq(&self, other: &QuestionIriOrLabel) -> bool {
		*self == QuestionIri || *self == QUESTION_LABEL
	}
}
