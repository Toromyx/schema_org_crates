/// <https://schema.org/subTest>
pub const SUB_TEST_PROPERTY_IRI_HTTP: &str = "http://schema.org/subTest";
/// <https://schema.org/subTest>
pub const SUB_TEST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/subTest";
/// <https://schema.org/subTest>
pub const SUB_TEST_PROPERTY_LABEL: &str = "subTest";
pub struct SubTestPropertyIri;
impl PartialEq<&str> for SubTestPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUB_TEST_PROPERTY_IRI_HTTP || *other == SUB_TEST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SubTestPropertyIri> for &str {
	fn eq(&self, other: &SubTestPropertyIri) -> bool {
		*self == SUB_TEST_PROPERTY_IRI_HTTP || *self == SUB_TEST_PROPERTY_IRI_HTTPS
	}
}
pub struct SubTestPropertyIriOrLabel;
impl PartialEq<&str> for SubTestPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubTestPropertyIri || *other == SUB_TEST_PROPERTY_LABEL
	}
}
impl PartialEq<SubTestPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SubTestPropertyIriOrLabel) -> bool {
		*self == SubTestPropertyIri || *self == SUB_TEST_PROPERTY_LABEL
	}
}
