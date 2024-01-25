/// <https://schema.org/partOfTVSeries>
#[deprecated = "This schema is superseded by <https://schema.org/partOfSeries>."]
pub const PART_OF_TV_SERIES_PROPERTY_IRI_HTTP: &str = "http://schema.org/partOfTVSeries";
/// <https://schema.org/partOfTVSeries>
#[deprecated = "This schema is superseded by <https://schema.org/partOfSeries>."]
pub const PART_OF_TV_SERIES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/partOfTVSeries";
/// <https://schema.org/partOfTVSeries>
#[deprecated = "This schema is superseded by <https://schema.org/partOfSeries>."]
pub const PART_OF_TV_SERIES_PROPERTY_LABEL: &str = "partOfTVSeries";
pub struct PartOfTvSeriesPropertyIri;
impl PartialEq<&str> for PartOfTvSeriesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PART_OF_TV_SERIES_PROPERTY_IRI_HTTP
			|| *other == PART_OF_TV_SERIES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PartOfTvSeriesPropertyIri> for &str {
	fn eq(&self, other: &PartOfTvSeriesPropertyIri) -> bool {
		*self == PART_OF_TV_SERIES_PROPERTY_IRI_HTTP
			|| *self == PART_OF_TV_SERIES_PROPERTY_IRI_HTTPS
	}
}
pub struct PartOfTvSeriesPropertyIriOrLabel;
impl PartialEq<&str> for PartOfTvSeriesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PartOfTvSeriesPropertyIri || *other == PART_OF_TV_SERIES_PROPERTY_LABEL
	}
}
impl PartialEq<PartOfTvSeriesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PartOfTvSeriesPropertyIriOrLabel) -> bool {
		*self == PartOfTvSeriesPropertyIri || *self == PART_OF_TV_SERIES_PROPERTY_LABEL
	}
}
