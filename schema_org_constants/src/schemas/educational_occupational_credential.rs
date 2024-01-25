/// <https://schema.org/EducationalOccupationalCredential>
pub const EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_IRI_HTTP: &str =
	"http://schema.org/EducationalOccupationalCredential";
/// <https://schema.org/EducationalOccupationalCredential>
pub const EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_IRI_HTTPS: &str =
	"https://schema.org/EducationalOccupationalCredential";
/// <https://schema.org/EducationalOccupationalCredential>
pub const EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_LABEL: &str = "EducationalOccupationalCredential";
pub struct EducationalOccupationalCredentialIri;
impl PartialEq<&str> for EducationalOccupationalCredentialIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_IRI_HTTP
			|| *other == EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_IRI_HTTPS
	}
}
impl PartialEq<EducationalOccupationalCredentialIri> for &str {
	fn eq(&self, other: &EducationalOccupationalCredentialIri) -> bool {
		*self == EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_IRI_HTTP
			|| *self == EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_IRI_HTTPS
	}
}
pub struct EducationalOccupationalCredentialIriOrLabel;
impl PartialEq<&str> for EducationalOccupationalCredentialIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalOccupationalCredentialIri
			|| *other == EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_LABEL
	}
}
impl PartialEq<EducationalOccupationalCredentialIriOrLabel> for &str {
	fn eq(&self, other: &EducationalOccupationalCredentialIriOrLabel) -> bool {
		*self == EducationalOccupationalCredentialIri
			|| *self == EDUCATIONAL_OCCUPATIONAL_CREDENTIAL_LABEL
	}
}
