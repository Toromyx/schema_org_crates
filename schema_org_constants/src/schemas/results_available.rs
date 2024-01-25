/// <https://schema.org/ResultsAvailable>
pub const RESULTS_AVAILABLE_IRI_HTTP: &str = "http://schema.org/ResultsAvailable";
/// <https://schema.org/ResultsAvailable>
pub const RESULTS_AVAILABLE_IRI_HTTPS: &str = "https://schema.org/ResultsAvailable";
/// <https://schema.org/ResultsAvailable>
pub const RESULTS_AVAILABLE_LABEL: &str = "ResultsAvailable";
pub struct ResultsAvailableIri;
impl PartialEq<&str> for ResultsAvailableIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESULTS_AVAILABLE_IRI_HTTP || *other == RESULTS_AVAILABLE_IRI_HTTPS
	}
}
impl PartialEq<ResultsAvailableIri> for &str {
	fn eq(&self, other: &ResultsAvailableIri) -> bool {
		*self == RESULTS_AVAILABLE_IRI_HTTP || *self == RESULTS_AVAILABLE_IRI_HTTPS
	}
}
pub struct ResultsAvailableIriOrLabel;
impl PartialEq<&str> for ResultsAvailableIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResultsAvailableIri || *other == RESULTS_AVAILABLE_LABEL
	}
}
impl PartialEq<ResultsAvailableIriOrLabel> for &str {
	fn eq(&self, other: &ResultsAvailableIriOrLabel) -> bool {
		*self == ResultsAvailableIri || *self == RESULTS_AVAILABLE_LABEL
	}
}
