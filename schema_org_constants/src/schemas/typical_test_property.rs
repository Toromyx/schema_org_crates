/// <https://schema.org/typicalTest>
pub const TYPICAL_TEST_PROPERTY_IRI_HTTP: &str = "http://schema.org/typicalTest";
/// <https://schema.org/typicalTest>
pub const TYPICAL_TEST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/typicalTest";
/// <https://schema.org/typicalTest>
pub const TYPICAL_TEST_PROPERTY_LABEL: &str = "typicalTest";
pub struct TypicalTestPropertyIri;
impl PartialEq<&str> for TypicalTestPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPICAL_TEST_PROPERTY_IRI_HTTP || *other == TYPICAL_TEST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TypicalTestPropertyIri> for &str {
	fn eq(&self, other: &TypicalTestPropertyIri) -> bool {
		*self == TYPICAL_TEST_PROPERTY_IRI_HTTP || *self == TYPICAL_TEST_PROPERTY_IRI_HTTPS
	}
}
pub struct TypicalTestPropertyIriOrLabel;
impl PartialEq<&str> for TypicalTestPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypicalTestPropertyIri || *other == TYPICAL_TEST_PROPERTY_LABEL
	}
}
impl PartialEq<TypicalTestPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TypicalTestPropertyIriOrLabel) -> bool {
		*self == TypicalTestPropertyIri || *self == TYPICAL_TEST_PROPERTY_LABEL
	}
}
