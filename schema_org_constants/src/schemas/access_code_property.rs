/// <https://schema.org/accessCode>
pub const ACCESS_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/accessCode";
/// <https://schema.org/accessCode>
pub const ACCESS_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accessCode";
/// <https://schema.org/accessCode>
pub const ACCESS_CODE_PROPERTY_LABEL: &str = "accessCode";
pub struct AccessCodePropertyIri;
impl PartialEq<&str> for AccessCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCESS_CODE_PROPERTY_IRI_HTTP || *other == ACCESS_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccessCodePropertyIri> for &str {
	fn eq(&self, other: &AccessCodePropertyIri) -> bool {
		*self == ACCESS_CODE_PROPERTY_IRI_HTTP || *self == ACCESS_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct AccessCodePropertyIriOrLabel;
impl PartialEq<&str> for AccessCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccessCodePropertyIri || *other == ACCESS_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<AccessCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccessCodePropertyIriOrLabel) -> bool {
		*self == AccessCodePropertyIri || *self == ACCESS_CODE_PROPERTY_LABEL
	}
}
