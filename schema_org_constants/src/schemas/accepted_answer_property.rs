/// <https://schema.org/acceptedAnswer>
pub const ACCEPTED_ANSWER_PROPERTY_IRI_HTTP: &str = "http://schema.org/acceptedAnswer";
/// <https://schema.org/acceptedAnswer>
pub const ACCEPTED_ANSWER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/acceptedAnswer";
/// <https://schema.org/acceptedAnswer>
pub const ACCEPTED_ANSWER_PROPERTY_LABEL: &str = "acceptedAnswer";
pub struct AcceptedAnswerPropertyIri;
impl PartialEq<&str> for AcceptedAnswerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCEPTED_ANSWER_PROPERTY_IRI_HTTP || *other == ACCEPTED_ANSWER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AcceptedAnswerPropertyIri> for &str {
	fn eq(&self, other: &AcceptedAnswerPropertyIri) -> bool {
		*self == ACCEPTED_ANSWER_PROPERTY_IRI_HTTP || *self == ACCEPTED_ANSWER_PROPERTY_IRI_HTTPS
	}
}
pub struct AcceptedAnswerPropertyIriOrLabel;
impl PartialEq<&str> for AcceptedAnswerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AcceptedAnswerPropertyIri || *other == ACCEPTED_ANSWER_PROPERTY_LABEL
	}
}
impl PartialEq<AcceptedAnswerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AcceptedAnswerPropertyIriOrLabel) -> bool {
		*self == AcceptedAnswerPropertyIri || *self == ACCEPTED_ANSWER_PROPERTY_LABEL
	}
}
