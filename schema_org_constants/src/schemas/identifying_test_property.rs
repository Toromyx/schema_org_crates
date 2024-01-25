/// <https://schema.org/identifyingTest>
pub const IDENTIFYING_TEST_PROPERTY_IRI_HTTP: &str = "http://schema.org/identifyingTest";
/// <https://schema.org/identifyingTest>
pub const IDENTIFYING_TEST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/identifyingTest";
/// <https://schema.org/identifyingTest>
pub const IDENTIFYING_TEST_PROPERTY_LABEL: &str = "identifyingTest";
pub struct IdentifyingTestPropertyIri;
impl PartialEq<&str> for IdentifyingTestPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IDENTIFYING_TEST_PROPERTY_IRI_HTTP
			|| *other == IDENTIFYING_TEST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IdentifyingTestPropertyIri> for &str {
	fn eq(&self, other: &IdentifyingTestPropertyIri) -> bool {
		*self == IDENTIFYING_TEST_PROPERTY_IRI_HTTP || *self == IDENTIFYING_TEST_PROPERTY_IRI_HTTPS
	}
}
pub struct IdentifyingTestPropertyIriOrLabel;
impl PartialEq<&str> for IdentifyingTestPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IdentifyingTestPropertyIri || *other == IDENTIFYING_TEST_PROPERTY_LABEL
	}
}
impl PartialEq<IdentifyingTestPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IdentifyingTestPropertyIriOrLabel) -> bool {
		*self == IdentifyingTestPropertyIri || *self == IDENTIFYING_TEST_PROPERTY_LABEL
	}
}
