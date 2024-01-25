/// <https://schema.org/Pathology>
pub const PATHOLOGY_IRI_HTTP: &str = "http://schema.org/Pathology";
/// <https://schema.org/Pathology>
pub const PATHOLOGY_IRI_HTTPS: &str = "https://schema.org/Pathology";
/// <https://schema.org/Pathology>
pub const PATHOLOGY_LABEL: &str = "Pathology";
pub struct PathologyIri;
impl PartialEq<&str> for PathologyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PATHOLOGY_IRI_HTTP || *other == PATHOLOGY_IRI_HTTPS
	}
}
impl PartialEq<PathologyIri> for &str {
	fn eq(&self, other: &PathologyIri) -> bool {
		*self == PATHOLOGY_IRI_HTTP || *self == PATHOLOGY_IRI_HTTPS
	}
}
pub struct PathologyIriOrLabel;
impl PartialEq<&str> for PathologyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PathologyIri || *other == PATHOLOGY_LABEL
	}
}
impl PartialEq<PathologyIriOrLabel> for &str {
	fn eq(&self, other: &PathologyIriOrLabel) -> bool {
		*self == PathologyIri || *self == PATHOLOGY_LABEL
	}
}
