/// <https://schema.org/ScreeningEvent>
pub const SCREENING_EVENT_IRI_HTTP: &str = "http://schema.org/ScreeningEvent";
/// <https://schema.org/ScreeningEvent>
pub const SCREENING_EVENT_IRI_HTTPS: &str = "https://schema.org/ScreeningEvent";
/// <https://schema.org/ScreeningEvent>
pub const SCREENING_EVENT_LABEL: &str = "ScreeningEvent";
pub struct ScreeningEventIri;
impl PartialEq<&str> for ScreeningEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCREENING_EVENT_IRI_HTTP || *other == SCREENING_EVENT_IRI_HTTPS
	}
}
impl PartialEq<ScreeningEventIri> for &str {
	fn eq(&self, other: &ScreeningEventIri) -> bool {
		*self == SCREENING_EVENT_IRI_HTTP || *self == SCREENING_EVENT_IRI_HTTPS
	}
}
pub struct ScreeningEventIriOrLabel;
impl PartialEq<&str> for ScreeningEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScreeningEventIri || *other == SCREENING_EVENT_LABEL
	}
}
impl PartialEq<ScreeningEventIriOrLabel> for &str {
	fn eq(&self, other: &ScreeningEventIriOrLabel) -> bool {
		*self == ScreeningEventIri || *self == SCREENING_EVENT_LABEL
	}
}
