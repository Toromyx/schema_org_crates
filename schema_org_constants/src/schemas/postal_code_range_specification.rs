/// <https://schema.org/PostalCodeRangeSpecification>
pub const POSTAL_CODE_RANGE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/PostalCodeRangeSpecification";
/// <https://schema.org/PostalCodeRangeSpecification>
pub const POSTAL_CODE_RANGE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/PostalCodeRangeSpecification";
/// <https://schema.org/PostalCodeRangeSpecification>
pub const POSTAL_CODE_RANGE_SPECIFICATION_LABEL: &str = "PostalCodeRangeSpecification";
pub struct PostalCodeRangeSpecificationIri;
impl PartialEq<&str> for PostalCodeRangeSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSTAL_CODE_RANGE_SPECIFICATION_IRI_HTTP
			|| *other == POSTAL_CODE_RANGE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<PostalCodeRangeSpecificationIri> for &str {
	fn eq(&self, other: &PostalCodeRangeSpecificationIri) -> bool {
		*self == POSTAL_CODE_RANGE_SPECIFICATION_IRI_HTTP
			|| *self == POSTAL_CODE_RANGE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct PostalCodeRangeSpecificationIriOrLabel;
impl PartialEq<&str> for PostalCodeRangeSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PostalCodeRangeSpecificationIri || *other == POSTAL_CODE_RANGE_SPECIFICATION_LABEL
	}
}
impl PartialEq<PostalCodeRangeSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &PostalCodeRangeSpecificationIriOrLabel) -> bool {
		*self == PostalCodeRangeSpecificationIri || *self == POSTAL_CODE_RANGE_SPECIFICATION_LABEL
	}
}
