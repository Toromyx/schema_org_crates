/// <https://schema.org/LiteraryEvent>
pub const LITERARY_EVENT_IRI_HTTP: &str = "http://schema.org/LiteraryEvent";
/// <https://schema.org/LiteraryEvent>
pub const LITERARY_EVENT_IRI_HTTPS: &str = "https://schema.org/LiteraryEvent";
/// <https://schema.org/LiteraryEvent>
pub const LITERARY_EVENT_LABEL: &str = "LiteraryEvent";
pub struct LiteraryEventIri;
impl PartialEq<&str> for LiteraryEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LITERARY_EVENT_IRI_HTTP || *other == LITERARY_EVENT_IRI_HTTPS
	}
}
impl PartialEq<LiteraryEventIri> for &str {
	fn eq(&self, other: &LiteraryEventIri) -> bool {
		*self == LITERARY_EVENT_IRI_HTTP || *self == LITERARY_EVENT_IRI_HTTPS
	}
}
pub struct LiteraryEventIriOrLabel;
impl PartialEq<&str> for LiteraryEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LiteraryEventIri || *other == LITERARY_EVENT_LABEL
	}
}
impl PartialEq<LiteraryEventIriOrLabel> for &str {
	fn eq(&self, other: &LiteraryEventIriOrLabel) -> bool {
		*self == LiteraryEventIri || *self == LITERARY_EVENT_LABEL
	}
}
