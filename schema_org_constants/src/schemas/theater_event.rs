/// <https://schema.org/TheaterEvent>
pub const THEATER_EVENT_IRI_HTTP: &str = "http://schema.org/TheaterEvent";
/// <https://schema.org/TheaterEvent>
pub const THEATER_EVENT_IRI_HTTPS: &str = "https://schema.org/TheaterEvent";
/// <https://schema.org/TheaterEvent>
pub const THEATER_EVENT_LABEL: &str = "TheaterEvent";
pub struct TheaterEventIri;
impl PartialEq<&str> for TheaterEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THEATER_EVENT_IRI_HTTP || *other == THEATER_EVENT_IRI_HTTPS
	}
}
impl PartialEq<TheaterEventIri> for &str {
	fn eq(&self, other: &TheaterEventIri) -> bool {
		*self == THEATER_EVENT_IRI_HTTP || *self == THEATER_EVENT_IRI_HTTPS
	}
}
pub struct TheaterEventIriOrLabel;
impl PartialEq<&str> for TheaterEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TheaterEventIri || *other == THEATER_EVENT_LABEL
	}
}
impl PartialEq<TheaterEventIriOrLabel> for &str {
	fn eq(&self, other: &TheaterEventIriOrLabel) -> bool {
		*self == TheaterEventIri || *self == THEATER_EVENT_LABEL
	}
}
