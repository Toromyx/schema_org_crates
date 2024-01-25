/// <https://schema.org/answerExplanation>
pub const ANSWER_EXPLANATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/answerExplanation";
/// <https://schema.org/answerExplanation>
pub const ANSWER_EXPLANATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/answerExplanation";
/// <https://schema.org/answerExplanation>
pub const ANSWER_EXPLANATION_PROPERTY_LABEL: &str = "answerExplanation";
pub struct AnswerExplanationPropertyIri;
impl PartialEq<&str> for AnswerExplanationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANSWER_EXPLANATION_PROPERTY_IRI_HTTP
			|| *other == ANSWER_EXPLANATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AnswerExplanationPropertyIri> for &str {
	fn eq(&self, other: &AnswerExplanationPropertyIri) -> bool {
		*self == ANSWER_EXPLANATION_PROPERTY_IRI_HTTP
			|| *self == ANSWER_EXPLANATION_PROPERTY_IRI_HTTPS
	}
}
pub struct AnswerExplanationPropertyIriOrLabel;
impl PartialEq<&str> for AnswerExplanationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnswerExplanationPropertyIri || *other == ANSWER_EXPLANATION_PROPERTY_LABEL
	}
}
impl PartialEq<AnswerExplanationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AnswerExplanationPropertyIriOrLabel) -> bool {
		*self == AnswerExplanationPropertyIri || *self == ANSWER_EXPLANATION_PROPERTY_LABEL
	}
}
