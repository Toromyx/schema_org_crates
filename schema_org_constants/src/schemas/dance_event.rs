/// <https://schema.org/DanceEvent>
pub const DANCE_EVENT_IRI_HTTP: &str = "http://schema.org/DanceEvent";
/// <https://schema.org/DanceEvent>
pub const DANCE_EVENT_IRI_HTTPS: &str = "https://schema.org/DanceEvent";
/// <https://schema.org/DanceEvent>
pub const DANCE_EVENT_LABEL: &str = "DanceEvent";
pub struct DanceEventIri;
impl PartialEq<&str> for DanceEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DANCE_EVENT_IRI_HTTP || *other == DANCE_EVENT_IRI_HTTPS
	}
}
impl PartialEq<DanceEventIri> for &str {
	fn eq(&self, other: &DanceEventIri) -> bool {
		*self == DANCE_EVENT_IRI_HTTP || *self == DANCE_EVENT_IRI_HTTPS
	}
}
pub struct DanceEventIriOrLabel;
impl PartialEq<&str> for DanceEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DanceEventIri || *other == DANCE_EVENT_LABEL
	}
}
impl PartialEq<DanceEventIriOrLabel> for &str {
	fn eq(&self, other: &DanceEventIriOrLabel) -> bool {
		*self == DanceEventIri || *self == DANCE_EVENT_LABEL
	}
}
