/// <https://schema.org/credentialCategory>
pub const CREDENTIAL_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/credentialCategory";
/// <https://schema.org/credentialCategory>
pub const CREDENTIAL_CATEGORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/credentialCategory";
/// <https://schema.org/credentialCategory>
pub const CREDENTIAL_CATEGORY_PROPERTY_LABEL: &str = "credentialCategory";
pub struct CredentialCategoryPropertyIri;
impl PartialEq<&str> for CredentialCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CREDENTIAL_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == CREDENTIAL_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CredentialCategoryPropertyIri> for &str {
	fn eq(&self, other: &CredentialCategoryPropertyIri) -> bool {
		*self == CREDENTIAL_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == CREDENTIAL_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct CredentialCategoryPropertyIriOrLabel;
impl PartialEq<&str> for CredentialCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CredentialCategoryPropertyIri || *other == CREDENTIAL_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<CredentialCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CredentialCategoryPropertyIriOrLabel) -> bool {
		*self == CredentialCategoryPropertyIri || *self == CREDENTIAL_CATEGORY_PROPERTY_LABEL
	}
}
