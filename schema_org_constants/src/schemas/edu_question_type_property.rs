/// <https://schema.org/eduQuestionType>
pub const EDU_QUESTION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/eduQuestionType";
/// <https://schema.org/eduQuestionType>
pub const EDU_QUESTION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/eduQuestionType";
/// <https://schema.org/eduQuestionType>
pub const EDU_QUESTION_TYPE_PROPERTY_LABEL: &str = "eduQuestionType";
pub struct EduQuestionTypePropertyIri;
impl PartialEq<&str> for EduQuestionTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDU_QUESTION_TYPE_PROPERTY_IRI_HTTP
			|| *other == EDU_QUESTION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EduQuestionTypePropertyIri> for &str {
	fn eq(&self, other: &EduQuestionTypePropertyIri) -> bool {
		*self == EDU_QUESTION_TYPE_PROPERTY_IRI_HTTP
			|| *self == EDU_QUESTION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct EduQuestionTypePropertyIriOrLabel;
impl PartialEq<&str> for EduQuestionTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EduQuestionTypePropertyIri || *other == EDU_QUESTION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<EduQuestionTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &EduQuestionTypePropertyIriOrLabel) -> bool {
		*self == EduQuestionTypePropertyIri || *self == EDU_QUESTION_TYPE_PROPERTY_LABEL
	}
}
