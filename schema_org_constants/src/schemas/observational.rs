/// <https://schema.org/Observational>
pub const OBSERVATIONAL_IRI_HTTP: &str = "http://schema.org/Observational";
/// <https://schema.org/Observational>
pub const OBSERVATIONAL_IRI_HTTPS: &str = "https://schema.org/Observational";
/// <https://schema.org/Observational>
pub const OBSERVATIONAL_LABEL: &str = "Observational";
pub struct ObservationalIri;
impl PartialEq<&str> for ObservationalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBSERVATIONAL_IRI_HTTP || *other == OBSERVATIONAL_IRI_HTTPS
	}
}
impl PartialEq<ObservationalIri> for &str {
	fn eq(&self, other: &ObservationalIri) -> bool {
		*self == OBSERVATIONAL_IRI_HTTP || *self == OBSERVATIONAL_IRI_HTTPS
	}
}
pub struct ObservationalIriOrLabel;
impl PartialEq<&str> for ObservationalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObservationalIri || *other == OBSERVATIONAL_LABEL
	}
}
impl PartialEq<ObservationalIriOrLabel> for &str {
	fn eq(&self, other: &ObservationalIriOrLabel) -> bool {
		*self == ObservationalIri || *self == OBSERVATIONAL_LABEL
	}
}
