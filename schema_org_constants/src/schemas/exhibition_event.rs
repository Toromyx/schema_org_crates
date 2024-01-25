/// <https://schema.org/ExhibitionEvent>
pub const EXHIBITION_EVENT_IRI_HTTP: &str = "http://schema.org/ExhibitionEvent";
/// <https://schema.org/ExhibitionEvent>
pub const EXHIBITION_EVENT_IRI_HTTPS: &str = "https://schema.org/ExhibitionEvent";
/// <https://schema.org/ExhibitionEvent>
pub const EXHIBITION_EVENT_LABEL: &str = "ExhibitionEvent";
pub struct ExhibitionEventIri;
impl PartialEq<&str> for ExhibitionEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXHIBITION_EVENT_IRI_HTTP || *other == EXHIBITION_EVENT_IRI_HTTPS
	}
}
impl PartialEq<ExhibitionEventIri> for &str {
	fn eq(&self, other: &ExhibitionEventIri) -> bool {
		*self == EXHIBITION_EVENT_IRI_HTTP || *self == EXHIBITION_EVENT_IRI_HTTPS
	}
}
pub struct ExhibitionEventIriOrLabel;
impl PartialEq<&str> for ExhibitionEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExhibitionEventIri || *other == EXHIBITION_EVENT_LABEL
	}
}
impl PartialEq<ExhibitionEventIriOrLabel> for &str {
	fn eq(&self, other: &ExhibitionEventIriOrLabel) -> bool {
		*self == ExhibitionEventIri || *self == EXHIBITION_EVENT_LABEL
	}
}
