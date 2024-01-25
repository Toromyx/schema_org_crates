/// <https://schema.org/postalCodeRange>
pub const POSTAL_CODE_RANGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/postalCodeRange";
/// <https://schema.org/postalCodeRange>
pub const POSTAL_CODE_RANGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/postalCodeRange";
/// <https://schema.org/postalCodeRange>
pub const POSTAL_CODE_RANGE_PROPERTY_LABEL: &str = "postalCodeRange";
pub struct PostalCodeRangePropertyIri;
impl PartialEq<&str> for PostalCodeRangePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_CODE_RANGE_PROPERTY_IRI_HTTP
			|| *other == POSTAL_CODE_RANGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PostalCodeRangePropertyIri> for &str {
	fn eq(&self, other: &PostalCodeRangePropertyIri) -> bool {
		*self == POSTAL_CODE_RANGE_PROPERTY_IRI_HTTP
			|| *self == POSTAL_CODE_RANGE_PROPERTY_IRI_HTTPS
	}
}
pub struct PostalCodeRangePropertyIriOrLabel;
impl PartialEq<&str> for PostalCodeRangePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalCodeRangePropertyIri || *other == POSTAL_CODE_RANGE_PROPERTY_LABEL
	}
}
impl PartialEq<PostalCodeRangePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PostalCodeRangePropertyIriOrLabel) -> bool {
		*self == PostalCodeRangePropertyIri || *self == POSTAL_CODE_RANGE_PROPERTY_LABEL
	}
}
