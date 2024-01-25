/// <https://schema.org/postalCode>
pub const POSTAL_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/postalCode";
/// <https://schema.org/postalCode>
pub const POSTAL_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/postalCode";
/// <https://schema.org/postalCode>
pub const POSTAL_CODE_PROPERTY_LABEL: &str = "postalCode";
pub struct PostalCodePropertyIri;
impl PartialEq<&str> for PostalCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_CODE_PROPERTY_IRI_HTTP || *other == POSTAL_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostalCodePropertyIri> for &str {
	fn eq(&self, other: &PostalCodePropertyIri) -> bool {
		*self == POSTAL_CODE_PROPERTY_IRI_HTTP || *self == POSTAL_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct PostalCodePropertyIriOrLabel;
impl PartialEq<&str> for PostalCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalCodePropertyIri || *other == POSTAL_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<PostalCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostalCodePropertyIriOrLabel) -> bool {
		*self == PostalCodePropertyIri || *self == POSTAL_CODE_PROPERTY_LABEL
	}
}
