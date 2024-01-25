/// <https://schema.org/RadioSeries>
pub const RADIO_SERIES_IRI_HTTP: &str = "http://schema.org/RadioSeries";
/// <https://schema.org/RadioSeries>
pub const RADIO_SERIES_IRI_HTTPS: &str = "https://schema.org/RadioSeries";
/// <https://schema.org/RadioSeries>
pub const RADIO_SERIES_LABEL: &str = "RadioSeries";
pub struct RadioSeriesIri;
impl PartialEq<&str> for RadioSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIO_SERIES_IRI_HTTP || *other == RADIO_SERIES_IRI_HTTPS
	}
}
impl PartialEq<RadioSeriesIri> for &str {
	fn eq(&self, other: &RadioSeriesIri) -> bool {
		*self == RADIO_SERIES_IRI_HTTP || *self == RADIO_SERIES_IRI_HTTPS
	}
}
pub struct RadioSeriesIriOrLabel;
impl PartialEq<&str> for RadioSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadioSeriesIri || *other == RADIO_SERIES_LABEL
	}
}
impl PartialEq<RadioSeriesIriOrLabel> for &str {
	fn eq(&self, other: &RadioSeriesIriOrLabel) -> bool {
		*self == RadioSeriesIri || *self == RADIO_SERIES_LABEL
	}
}
