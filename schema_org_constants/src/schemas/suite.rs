/// <https://schema.org/Suite>
pub const SUITE_IRI_HTTP: &str = "http://schema.org/Suite";
/// <https://schema.org/Suite>
pub const SUITE_IRI_HTTPS: &str = "https://schema.org/Suite";
/// <https://schema.org/Suite>
pub const SUITE_LABEL: &str = "Suite";
pub struct SuiteIri;
impl PartialEq<&str> for SuiteIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUITE_IRI_HTTP || *other == SUITE_IRI_HTTPS
	}
}
impl PartialEq<SuiteIri> for &str {
	fn eq(&self, other: &SuiteIri) -> bool {
		*self == SUITE_IRI_HTTP || *self == SUITE_IRI_HTTPS
	}
}
pub struct SuiteIriOrLabel;
impl PartialEq<&str> for SuiteIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuiteIri || *other == SUITE_LABEL
	}
}
impl PartialEq<SuiteIriOrLabel> for &str {
	fn eq(&self, other: &SuiteIriOrLabel) -> bool {
		*self == SuiteIri || *self == SUITE_LABEL
	}
}
