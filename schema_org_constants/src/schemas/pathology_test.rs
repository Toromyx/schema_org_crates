/// <https://schema.org/PathologyTest>
pub const PATHOLOGY_TEST_IRI_HTTP: &str = "http://schema.org/PathologyTest";
/// <https://schema.org/PathologyTest>
pub const PATHOLOGY_TEST_IRI_HTTPS: &str = "https://schema.org/PathologyTest";
/// <https://schema.org/PathologyTest>
pub const PATHOLOGY_TEST_LABEL: &str = "PathologyTest";
pub struct PathologyTestIri;
impl PartialEq<&str> for PathologyTestIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PATHOLOGY_TEST_IRI_HTTP || *other == PATHOLOGY_TEST_IRI_HTTPS
	}
}
impl PartialEq<PathologyTestIri> for &str {
	fn eq(&self, other: &PathologyTestIri) -> bool {
		*self == PATHOLOGY_TEST_IRI_HTTP || *self == PATHOLOGY_TEST_IRI_HTTPS
	}
}
pub struct PathologyTestIriOrLabel;
impl PartialEq<&str> for PathologyTestIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PathologyTestIri || *other == PATHOLOGY_TEST_LABEL
	}
}
impl PartialEq<PathologyTestIriOrLabel> for &str {
	fn eq(&self, other: &PathologyTestIriOrLabel) -> bool {
		*self == PathologyTestIri || *self == PATHOLOGY_TEST_LABEL
	}
}
