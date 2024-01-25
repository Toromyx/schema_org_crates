/// <https://schema.org/CaseSeries>
pub const CASE_SERIES_IRI_HTTP: &str = "http://schema.org/CaseSeries";
/// <https://schema.org/CaseSeries>
pub const CASE_SERIES_IRI_HTTPS: &str = "https://schema.org/CaseSeries";
/// <https://schema.org/CaseSeries>
pub const CASE_SERIES_LABEL: &str = "CaseSeries";
pub struct CaseSeriesIri;
impl PartialEq<&str> for CaseSeriesIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CASE_SERIES_IRI_HTTP || *other == CASE_SERIES_IRI_HTTPS
	}
}
impl PartialEq<CaseSeriesIri> for &str {
	fn eq(&self, other: &CaseSeriesIri) -> bool {
		*self == CASE_SERIES_IRI_HTTP || *self == CASE_SERIES_IRI_HTTPS
	}
}
pub struct CaseSeriesIriOrLabel;
impl PartialEq<&str> for CaseSeriesIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CaseSeriesIri || *other == CASE_SERIES_LABEL
	}
}
impl PartialEq<CaseSeriesIriOrLabel> for &str {
	fn eq(&self, other: &CaseSeriesIriOrLabel) -> bool {
		*self == CaseSeriesIri || *self == CASE_SERIES_LABEL
	}
}
