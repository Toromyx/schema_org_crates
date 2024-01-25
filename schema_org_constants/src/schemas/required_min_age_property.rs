/// <https://schema.org/requiredMinAge>
pub const REQUIRED_MIN_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/requiredMinAge";
/// <https://schema.org/requiredMinAge>
pub const REQUIRED_MIN_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/requiredMinAge";
/// <https://schema.org/requiredMinAge>
pub const REQUIRED_MIN_AGE_PROPERTY_LABEL: &str = "requiredMinAge";
pub struct RequiredMinAgePropertyIri;
impl PartialEq<&str> for RequiredMinAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIRED_MIN_AGE_PROPERTY_IRI_HTTP
			|| *other == REQUIRED_MIN_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequiredMinAgePropertyIri> for &str {
	fn eq(&self, other: &RequiredMinAgePropertyIri) -> bool {
		*self == REQUIRED_MIN_AGE_PROPERTY_IRI_HTTP || *self == REQUIRED_MIN_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct RequiredMinAgePropertyIriOrLabel;
impl PartialEq<&str> for RequiredMinAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequiredMinAgePropertyIri || *other == REQUIRED_MIN_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<RequiredMinAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequiredMinAgePropertyIriOrLabel) -> bool {
		*self == RequiredMinAgePropertyIri || *self == REQUIRED_MIN_AGE_PROPERTY_LABEL
	}
}
