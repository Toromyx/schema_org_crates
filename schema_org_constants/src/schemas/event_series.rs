/// <https://schema.org/EventSeries>
pub const EVENT_SERIES_IRI_HTTP: &str = "http://schema.org/EventSeries";
/// <https://schema.org/EventSeries>
pub const EVENT_SERIES_IRI_HTTPS: &str = "https://schema.org/EventSeries";
/// <https://schema.org/EventSeries>
pub const EVENT_SERIES_LABEL: &str = "EventSeries";
pub struct EventSeriesIri;
impl PartialEq<&str> for EventSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EVENT_SERIES_IRI_HTTP || *other == EVENT_SERIES_IRI_HTTPS
	}
}
impl PartialEq<EventSeriesIri> for &str {
	fn eq(&self, other: &EventSeriesIri) -> bool {
		*self == EVENT_SERIES_IRI_HTTP || *self == EVENT_SERIES_IRI_HTTPS
	}
}
pub struct EventSeriesIriOrLabel;
impl PartialEq<&str> for EventSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EventSeriesIri || *other == EVENT_SERIES_LABEL
	}
}
impl PartialEq<EventSeriesIriOrLabel> for &str {
	fn eq(&self, other: &EventSeriesIriOrLabel) -> bool {
		*self == EventSeriesIri || *self == EVENT_SERIES_LABEL
	}
}
