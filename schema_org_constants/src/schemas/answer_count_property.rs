/// <https://schema.org/answerCount>
pub const ANSWER_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/answerCount";
/// <https://schema.org/answerCount>
pub const ANSWER_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/answerCount";
/// <https://schema.org/answerCount>
pub const ANSWER_COUNT_PROPERTY_LABEL: &str = "answerCount";
pub struct AnswerCountPropertyIri;
impl PartialEq<&str> for AnswerCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANSWER_COUNT_PROPERTY_IRI_HTTP || *other == ANSWER_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AnswerCountPropertyIri> for &str {
	fn eq(&self, other: &AnswerCountPropertyIri) -> bool {
		*self == ANSWER_COUNT_PROPERTY_IRI_HTTP || *self == ANSWER_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct AnswerCountPropertyIriOrLabel;
impl PartialEq<&str> for AnswerCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnswerCountPropertyIri || *other == ANSWER_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<AnswerCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AnswerCountPropertyIriOrLabel) -> bool {
		*self == AnswerCountPropertyIri || *self == ANSWER_COUNT_PROPERTY_LABEL
	}
}
