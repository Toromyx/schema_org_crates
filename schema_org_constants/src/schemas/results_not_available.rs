/// <https://schema.org/ResultsNotAvailable>
pub const RESULTS_NOT_AVAILABLE_IRI_HTTP: &str = "http://schema.org/ResultsNotAvailable";
/// <https://schema.org/ResultsNotAvailable>
pub const RESULTS_NOT_AVAILABLE_IRI_HTTPS: &str = "https://schema.org/ResultsNotAvailable";
/// <https://schema.org/ResultsNotAvailable>
pub const RESULTS_NOT_AVAILABLE_LABEL: &str = "ResultsNotAvailable";
pub struct ResultsNotAvailableIri;
impl PartialEq<&str> for ResultsNotAvailableIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESULTS_NOT_AVAILABLE_IRI_HTTP || *other == RESULTS_NOT_AVAILABLE_IRI_HTTPS
	}
}
impl PartialEq<ResultsNotAvailableIri> for &str {
	fn eq(&self, other: &ResultsNotAvailableIri) -> bool {
		*self == RESULTS_NOT_AVAILABLE_IRI_HTTP || *self == RESULTS_NOT_AVAILABLE_IRI_HTTPS
	}
}
pub struct ResultsNotAvailableIriOrLabel;
impl PartialEq<&str> for ResultsNotAvailableIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResultsNotAvailableIri || *other == RESULTS_NOT_AVAILABLE_LABEL
	}
}
impl PartialEq<ResultsNotAvailableIriOrLabel> for &str {
	fn eq(&self, other: &ResultsNotAvailableIriOrLabel) -> bool {
		*self == ResultsNotAvailableIri || *self == RESULTS_NOT_AVAILABLE_LABEL
	}
}
