/// <https://schema.org/Quiz>
pub const QUIZ_IRI_HTTP: &str = "http://schema.org/Quiz";
/// <https://schema.org/Quiz>
pub const QUIZ_IRI_HTTPS: &str = "https://schema.org/Quiz";
/// <https://schema.org/Quiz>
pub const QUIZ_LABEL: &str = "Quiz";
pub struct QuizIri;
impl PartialEq<&str> for QuizIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUIZ_IRI_HTTP || *other == QUIZ_IRI_HTTPS
	}
}
impl PartialEq<QuizIri> for &str {
	fn eq(&self, other: &QuizIri) -> bool {
		*self == QUIZ_IRI_HTTP || *self == QUIZ_IRI_HTTPS
	}
}
pub struct QuizIriOrLabel;
impl PartialEq<&str> for QuizIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QuizIri || *other == QUIZ_LABEL
	}
}
impl PartialEq<QuizIriOrLabel> for &str {
	fn eq(&self, other: &QuizIriOrLabel) -> bool {
		*self == QuizIri || *self == QUIZ_LABEL
	}
}
