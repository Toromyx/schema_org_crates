/// <https://schema.org/postalCodeEnd>
pub const POSTAL_CODE_END_PROPERTY_IRI_HTTP: &str = "http://schema.org/postalCodeEnd";
/// <https://schema.org/postalCodeEnd>
pub const POSTAL_CODE_END_PROPERTY_IRI_HTTPS: &str = "https://schema.org/postalCodeEnd";
/// <https://schema.org/postalCodeEnd>
pub const POSTAL_CODE_END_PROPERTY_LABEL: &str = "postalCodeEnd";
pub struct PostalCodeEndPropertyIri;
impl PartialEq<&str> for PostalCodeEndPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_CODE_END_PROPERTY_IRI_HTTP || *other == POSTAL_CODE_END_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostalCodeEndPropertyIri> for &str {
	fn eq(&self, other: &PostalCodeEndPropertyIri) -> bool {
		*self == POSTAL_CODE_END_PROPERTY_IRI_HTTP || *self == POSTAL_CODE_END_PROPERTY_IRI_HTTPS
	}
}
pub struct PostalCodeEndPropertyIriOrLabel;
impl PartialEq<&str> for PostalCodeEndPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalCodeEndPropertyIri || *other == POSTAL_CODE_END_PROPERTY_LABEL
	}
}
impl PartialEq<PostalCodeEndPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostalCodeEndPropertyIriOrLabel) -> bool {
		*self == PostalCodeEndPropertyIri || *self == POSTAL_CODE_END_PROPERTY_LABEL
	}
}
