/// <https://schema.org/postalCodePrefix>
pub const POSTAL_CODE_PREFIX_PROPERTY_IRI_HTTP: &str = "http://schema.org/postalCodePrefix";
/// <https://schema.org/postalCodePrefix>
pub const POSTAL_CODE_PREFIX_PROPERTY_IRI_HTTPS: &str = "https://schema.org/postalCodePrefix";
/// <https://schema.org/postalCodePrefix>
pub const POSTAL_CODE_PREFIX_PROPERTY_LABEL: &str = "postalCodePrefix";
pub struct PostalCodePrefixPropertyIri;
impl PartialEq<&str> for PostalCodePrefixPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_CODE_PREFIX_PROPERTY_IRI_HTTP
			|| *other == POSTAL_CODE_PREFIX_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostalCodePrefixPropertyIri> for &str {
	fn eq(&self, other: &PostalCodePrefixPropertyIri) -> bool {
		*self == POSTAL_CODE_PREFIX_PROPERTY_IRI_HTTP
			|| *self == POSTAL_CODE_PREFIX_PROPERTY_IRI_HTTPS
	}
}
pub struct PostalCodePrefixPropertyIriOrLabel;
impl PartialEq<&str> for PostalCodePrefixPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalCodePrefixPropertyIri || *other == POSTAL_CODE_PREFIX_PROPERTY_LABEL
	}
}
impl PartialEq<PostalCodePrefixPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostalCodePrefixPropertyIriOrLabel) -> bool {
		*self == PostalCodePrefixPropertyIri || *self == POSTAL_CODE_PREFIX_PROPERTY_LABEL
	}
}
