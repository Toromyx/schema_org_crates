/// <https://schema.org/postalCodeBegin>
pub const POSTAL_CODE_BEGIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/postalCodeBegin";
/// <https://schema.org/postalCodeBegin>
pub const POSTAL_CODE_BEGIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/postalCodeBegin";
/// <https://schema.org/postalCodeBegin>
pub const POSTAL_CODE_BEGIN_PROPERTY_LABEL: &str = "postalCodeBegin";
pub struct PostalCodeBeginPropertyIri;
impl PartialEq<&str> for PostalCodeBeginPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_CODE_BEGIN_PROPERTY_IRI_HTTP
			|| *other == POSTAL_CODE_BEGIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostalCodeBeginPropertyIri> for &str {
	fn eq(&self, other: &PostalCodeBeginPropertyIri) -> bool {
		*self == POSTAL_CODE_BEGIN_PROPERTY_IRI_HTTP
			|| *self == POSTAL_CODE_BEGIN_PROPERTY_IRI_HTTPS
	}
}
pub struct PostalCodeBeginPropertyIriOrLabel;
impl PartialEq<&str> for PostalCodeBeginPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalCodeBeginPropertyIri || *other == POSTAL_CODE_BEGIN_PROPERTY_LABEL
	}
}
impl PartialEq<PostalCodeBeginPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostalCodeBeginPropertyIriOrLabel) -> bool {
		*self == PostalCodeBeginPropertyIri || *self == POSTAL_CODE_BEGIN_PROPERTY_LABEL
	}
}
