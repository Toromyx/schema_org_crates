/// <https://schema.org/suggestedAnswer>
pub const SUGGESTED_ANSWER_PROPERTY_IRI_HTTP: &str = "http://schema.org/suggestedAnswer";
/// <https://schema.org/suggestedAnswer>
pub const SUGGESTED_ANSWER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/suggestedAnswer";
/// <https://schema.org/suggestedAnswer>
pub const SUGGESTED_ANSWER_PROPERTY_LABEL: &str = "suggestedAnswer";
pub struct SuggestedAnswerPropertyIri;
impl PartialEq<&str> for SuggestedAnswerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUGGESTED_ANSWER_PROPERTY_IRI_HTTP
			|| *other == SUGGESTED_ANSWER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuggestedAnswerPropertyIri> for &str {
	fn eq(&self, other: &SuggestedAnswerPropertyIri) -> bool {
		*self == SUGGESTED_ANSWER_PROPERTY_IRI_HTTP || *self == SUGGESTED_ANSWER_PROPERTY_IRI_HTTPS
	}
}
pub struct SuggestedAnswerPropertyIriOrLabel;
impl PartialEq<&str> for SuggestedAnswerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuggestedAnswerPropertyIri || *other == SUGGESTED_ANSWER_PROPERTY_LABEL
	}
}
impl PartialEq<SuggestedAnswerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuggestedAnswerPropertyIriOrLabel) -> bool {
		*self == SuggestedAnswerPropertyIri || *self == SUGGESTED_ANSWER_PROPERTY_LABEL
	}
}
