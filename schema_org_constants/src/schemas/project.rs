/// <https://schema.org/Project>
pub const PROJECT_IRI_HTTP: &str = "http://schema.org/Project";
/// <https://schema.org/Project>
pub const PROJECT_IRI_HTTPS: &str = "https://schema.org/Project";
/// <https://schema.org/Project>
pub const PROJECT_LABEL: &str = "Project";
pub struct ProjectIri;
impl PartialEq<&str> for ProjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROJECT_IRI_HTTP || *other == PROJECT_IRI_HTTPS
	}
}
impl PartialEq<ProjectIri> for &str {
	fn eq(&self, other: &ProjectIri) -> bool {
		*self == PROJECT_IRI_HTTP || *self == PROJECT_IRI_HTTPS
	}
}
pub struct ProjectIriOrLabel;
impl PartialEq<&str> for ProjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProjectIri || *other == PROJECT_LABEL
	}
}
impl PartialEq<ProjectIriOrLabel> for &str {
	fn eq(&self, other: &ProjectIriOrLabel) -> bool {
		*self == ProjectIri || *self == PROJECT_LABEL
	}
}
