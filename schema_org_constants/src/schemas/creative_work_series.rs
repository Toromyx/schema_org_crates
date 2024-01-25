/// <https://schema.org/CreativeWorkSeries>
pub const CREATIVE_WORK_SERIES_IRI_HTTP: &str = "http://schema.org/CreativeWorkSeries";
/// <https://schema.org/CreativeWorkSeries>
pub const CREATIVE_WORK_SERIES_IRI_HTTPS: &str = "https://schema.org/CreativeWorkSeries";
/// <https://schema.org/CreativeWorkSeries>
pub const CREATIVE_WORK_SERIES_LABEL: &str = "CreativeWorkSeries";
pub struct CreativeWorkSeriesIri;
impl PartialEq<&str> for CreativeWorkSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREATIVE_WORK_SERIES_IRI_HTTP || *other == CREATIVE_WORK_SERIES_IRI_HTTPS
	}
}
impl PartialEq<CreativeWorkSeriesIri> for &str {
	fn eq(&self, other: &CreativeWorkSeriesIri) -> bool {
		*self == CREATIVE_WORK_SERIES_IRI_HTTP || *self == CREATIVE_WORK_SERIES_IRI_HTTPS
	}
}
pub struct CreativeWorkSeriesIriOrLabel;
impl PartialEq<&str> for CreativeWorkSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CreativeWorkSeriesIri || *other == CREATIVE_WORK_SERIES_LABEL
	}
}
impl PartialEq<CreativeWorkSeriesIriOrLabel> for &str {
	fn eq(&self, other: &CreativeWorkSeriesIriOrLabel) -> bool {
		*self == CreativeWorkSeriesIri || *self == CREATIVE_WORK_SERIES_LABEL
	}
}
