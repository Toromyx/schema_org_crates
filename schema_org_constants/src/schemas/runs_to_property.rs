/// <https://schema.org/runsTo>
pub const RUNS_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/runsTo";
/// <https://schema.org/runsTo>
pub const RUNS_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/runsTo";
/// <https://schema.org/runsTo>
pub const RUNS_TO_PROPERTY_LABEL: &str = "runsTo";
pub struct RunsToPropertyIri;
impl PartialEq<&str> for RunsToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RUNS_TO_PROPERTY_IRI_HTTP || *other == RUNS_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RunsToPropertyIri> for &str {
	fn eq(&self, other: &RunsToPropertyIri) -> bool {
		*self == RUNS_TO_PROPERTY_IRI_HTTP || *self == RUNS_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct RunsToPropertyIriOrLabel;
impl PartialEq<&str> for RunsToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RunsToPropertyIri || *other == RUNS_TO_PROPERTY_LABEL
	}
}
impl PartialEq<RunsToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RunsToPropertyIriOrLabel) -> bool {
		*self == RunsToPropertyIri || *self == RUNS_TO_PROPERTY_LABEL
	}
}
