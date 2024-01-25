/// <https://schema.org/availableTest>
pub const AVAILABLE_TEST_PROPERTY_IRI_HTTP: &str = "http://schema.org/availableTest";
/// <https://schema.org/availableTest>
pub const AVAILABLE_TEST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/availableTest";
/// <https://schema.org/availableTest>
pub const AVAILABLE_TEST_PROPERTY_LABEL: &str = "availableTest";
pub struct AvailableTestPropertyIri;
impl PartialEq<&str> for AvailableTestPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AVAILABLE_TEST_PROPERTY_IRI_HTTP || *other == AVAILABLE_TEST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AvailableTestPropertyIri> for &str {
	fn eq(&self, other: &AvailableTestPropertyIri) -> bool {
		*self == AVAILABLE_TEST_PROPERTY_IRI_HTTP || *self == AVAILABLE_TEST_PROPERTY_IRI_HTTPS
	}
}
pub struct AvailableTestPropertyIriOrLabel;
impl PartialEq<&str> for AvailableTestPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AvailableTestPropertyIri || *other == AVAILABLE_TEST_PROPERTY_LABEL
	}
}
impl PartialEq<AvailableTestPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AvailableTestPropertyIriOrLabel) -> bool {
		*self == AvailableTestPropertyIri || *self == AVAILABLE_TEST_PROPERTY_LABEL
	}
}
