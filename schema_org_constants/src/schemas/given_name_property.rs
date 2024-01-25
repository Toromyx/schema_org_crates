/// <https://schema.org/givenName>
pub const GIVEN_NAME_PROPERTY_IRI_HTTP: &str = "http://schema.org/givenName";
/// <https://schema.org/givenName>
pub const GIVEN_NAME_PROPERTY_IRI_HTTPS: &str = "https://schema.org/givenName";
/// <https://schema.org/givenName>
pub const GIVEN_NAME_PROPERTY_LABEL: &str = "givenName";
pub struct GivenNamePropertyIri;
impl PartialEq<&str> for GivenNamePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GIVEN_NAME_PROPERTY_IRI_HTTP || *other == GIVEN_NAME_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GivenNamePropertyIri> for &str {
	fn eq(&self, other: &GivenNamePropertyIri) -> bool {
		*self == GIVEN_NAME_PROPERTY_IRI_HTTP || *self == GIVEN_NAME_PROPERTY_IRI_HTTPS
	}
}
pub struct GivenNamePropertyIriOrLabel;
impl PartialEq<&str> for GivenNamePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GivenNamePropertyIri || *other == GIVEN_NAME_PROPERTY_LABEL
	}
}
impl PartialEq<GivenNamePropertyIriOrLabel> for &str {
	fn eq(&self, other: &GivenNamePropertyIriOrLabel) -> bool {
		*self == GivenNamePropertyIri || *self == GIVEN_NAME_PROPERTY_LABEL
	}
}
