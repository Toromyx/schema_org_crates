/// <https://schema.org/Series>
pub const SERIES_IRI_HTTP: &str = "http://schema.org/Series";
/// <https://schema.org/Series>
pub const SERIES_IRI_HTTPS: &str = "https://schema.org/Series";
/// <https://schema.org/Series>
pub const SERIES_LABEL: &str = "Series";
pub struct SeriesIri;
impl PartialEq<&str> for SeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERIES_IRI_HTTP || *other == SERIES_IRI_HTTPS
	}
}
impl PartialEq<SeriesIri> for &str {
	fn eq(&self, other: &SeriesIri) -> bool {
		*self == SERIES_IRI_HTTP || *self == SERIES_IRI_HTTPS
	}
}
pub struct SeriesIriOrLabel;
impl PartialEq<&str> for SeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeriesIri || *other == SERIES_LABEL
	}
}
impl PartialEq<SeriesIriOrLabel> for &str {
	fn eq(&self, other: &SeriesIriOrLabel) -> bool {
		*self == SeriesIri || *self == SERIES_LABEL
	}
}
