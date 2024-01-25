/// <https://schema.org/question>
pub const QUESTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/question";
/// <https://schema.org/question>
pub const QUESTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/question";
/// <https://schema.org/question>
pub const QUESTION_PROPERTY_LABEL: &str = "question";
pub struct QuestionPropertyIri;
impl PartialEq<&str> for QuestionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUESTION_PROPERTY_IRI_HTTP || *other == QUESTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<QuestionPropertyIri> for &str {
	fn eq(&self, other: &QuestionPropertyIri) -> bool {
		*self == QUESTION_PROPERTY_IRI_HTTP || *self == QUESTION_PROPERTY_IRI_HTTPS
	}
}
pub struct QuestionPropertyIriOrLabel;
impl PartialEq<&str> for QuestionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuestionPropertyIri || *other == QUESTION_PROPERTY_LABEL
	}
}
impl PartialEq<QuestionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &QuestionPropertyIriOrLabel) -> bool {
		*self == QuestionPropertyIri || *self == QUESTION_PROPERTY_LABEL
	}
}
