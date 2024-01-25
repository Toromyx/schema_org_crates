/// <https://schema.org/requiredMaxAge>
pub const REQUIRED_MAX_AGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/requiredMaxAge";
/// <https://schema.org/requiredMaxAge>
pub const REQUIRED_MAX_AGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/requiredMaxAge";
/// <https://schema.org/requiredMaxAge>
pub const REQUIRED_MAX_AGE_PROPERTY_LABEL: &str = "requiredMaxAge";
pub struct RequiredMaxAgePropertyIri;
impl PartialEq<&str> for RequiredMaxAgePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIRED_MAX_AGE_PROPERTY_IRI_HTTP
			|| *other == REQUIRED_MAX_AGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequiredMaxAgePropertyIri> for &str {
	fn eq(&self, other: &RequiredMaxAgePropertyIri) -> bool {
		*self == REQUIRED_MAX_AGE_PROPERTY_IRI_HTTP || *self == REQUIRED_MAX_AGE_PROPERTY_IRI_HTTPS
	}
}
pub struct RequiredMaxAgePropertyIriOrLabel;
impl PartialEq<&str> for RequiredMaxAgePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequiredMaxAgePropertyIri || *other == REQUIRED_MAX_AGE_PROPERTY_LABEL
	}
}
impl PartialEq<RequiredMaxAgePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequiredMaxAgePropertyIriOrLabel) -> bool {
		*self == RequiredMaxAgePropertyIri || *self == REQUIRED_MAX_AGE_PROPERTY_LABEL
	}
}
