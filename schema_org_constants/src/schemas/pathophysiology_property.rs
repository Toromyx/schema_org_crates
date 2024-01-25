/// <https://schema.org/pathophysiology>
pub const PATHOPHYSIOLOGY_PROPERTY_IRI_HTTP: &str = "http://schema.org/pathophysiology";
/// <https://schema.org/pathophysiology>
pub const PATHOPHYSIOLOGY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pathophysiology";
/// <https://schema.org/pathophysiology>
pub const PATHOPHYSIOLOGY_PROPERTY_LABEL: &str = "pathophysiology";
pub struct PathophysiologyPropertyIri;
impl PartialEq<&str> for PathophysiologyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PATHOPHYSIOLOGY_PROPERTY_IRI_HTTP || *other == PATHOPHYSIOLOGY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PathophysiologyPropertyIri> for &str {
	fn eq(&self, other: &PathophysiologyPropertyIri) -> bool {
		*self == PATHOPHYSIOLOGY_PROPERTY_IRI_HTTP || *self == PATHOPHYSIOLOGY_PROPERTY_IRI_HTTPS
	}
}
pub struct PathophysiologyPropertyIriOrLabel;
impl PartialEq<&str> for PathophysiologyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PathophysiologyPropertyIri || *other == PATHOPHYSIOLOGY_PROPERTY_LABEL
	}
}
impl PartialEq<PathophysiologyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PathophysiologyPropertyIriOrLabel) -> bool {
		*self == PathophysiologyPropertyIri || *self == PATHOPHYSIOLOGY_PROPERTY_LABEL
	}
}
