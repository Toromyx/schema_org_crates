/// <https://schema.org/partOfSeries>
pub const PART_OF_SERIES_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfSeries";
/// <https://schema.org/partOfSeries>
pub const PART_OF_SERIES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfSeries";
/// <https://schema.org/partOfSeries>
pub const PART_OF_SERIES_PROPERTY_LABEL: &str = "partOfSeries";
pub struct PartOfSeriesPropertyIri;
impl PartialEq<&str> for PartOfSeriesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_SERIES_PROPERTY_IRI_HTTP || *other == PART_OF_SERIES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfSeriesPropertyIri> for &str {
	fn eq(&self, other: &PartOfSeriesPropertyIri) -> bool {
		*self == PART_OF_SERIES_PROPERTY_IRI_HTTP || *self == PART_OF_SERIES_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfSeriesPropertyIriOrLabel;
impl PartialEq<&str> for PartOfSeriesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfSeriesPropertyIri || *other == PART_OF_SERIES_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfSeriesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfSeriesPropertyIriOrLabel) -> bool {
		*self == PartOfSeriesPropertyIri || *self == PART_OF_SERIES_PROPERTY_LABEL
	}
}
