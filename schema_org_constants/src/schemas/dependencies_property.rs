/// <https://schema.org/dependencies>
pub const DEPENDENCIES_PROPERTY_IRI_HTTP: &str = "http://schema.org/dependencies";
/// <https://schema.org/dependencies>
pub const DEPENDENCIES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dependencies";
/// <https://schema.org/dependencies>
pub const DEPENDENCIES_PROPERTY_LABEL: &str = "dependencies";
pub struct DependenciesPropertyIri;
impl PartialEq<&str> for DependenciesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPENDENCIES_PROPERTY_IRI_HTTP || *other == DEPENDENCIES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DependenciesPropertyIri> for &str {
	fn eq(&self, other: &DependenciesPropertyIri) -> bool {
		*self == DEPENDENCIES_PROPERTY_IRI_HTTP || *self == DEPENDENCIES_PROPERTY_IRI_HTTPS
	}
}
pub struct DependenciesPropertyIriOrLabel;
impl PartialEq<&str> for DependenciesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DependenciesPropertyIri || *other == DEPENDENCIES_PROPERTY_LABEL
	}
}
impl PartialEq<DependenciesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DependenciesPropertyIriOrLabel) -> bool {
		*self == DependenciesPropertyIri || *self == DEPENDENCIES_PROPERTY_LABEL
	}
}
