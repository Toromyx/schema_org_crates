/// <https://schema.org/educationalCredentialAwarded>
pub const EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/educationalCredentialAwarded";
/// <https://schema.org/educationalCredentialAwarded>
pub const EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/educationalCredentialAwarded";
/// <https://schema.org/educationalCredentialAwarded>
pub const EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_LABEL: &str = "educationalCredentialAwarded";
pub struct EducationalCredentialAwardedPropertyIri;
impl PartialEq<&str> for EducationalCredentialAwardedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTP
			|| *other == EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EducationalCredentialAwardedPropertyIri> for &str {
	fn eq(&self, other: &EducationalCredentialAwardedPropertyIri) -> bool {
		*self == EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTP
			|| *self == EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_IRI_HTTPS
	}
}
pub struct EducationalCredentialAwardedPropertyIriOrLabel;
impl PartialEq<&str> for EducationalCredentialAwardedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalCredentialAwardedPropertyIri
			|| *other == EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_LABEL
	}
}
impl PartialEq<EducationalCredentialAwardedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EducationalCredentialAwardedPropertyIriOrLabel) -> bool {
		*self == EducationalCredentialAwardedPropertyIri
			|| *self == EDUCATIONAL_CREDENTIAL_AWARDED_PROPERTY_LABEL
	}
}
