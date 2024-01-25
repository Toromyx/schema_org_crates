/// <https://schema.org/hasCredential>
pub const HAS_CREDENTIAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/hasCredential";
/// <https://schema.org/hasCredential>
pub const HAS_CREDENTIAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/hasCredential";
/// <https://schema.org/hasCredential>
pub const HAS_CREDENTIAL_PROPERTY_LABEL: &str = "hasCredential";
pub struct HasCredentialPropertyIri;
impl PartialEq<&str> for HasCredentialPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_CREDENTIAL_PROPERTY_IRI_HTTP || *other == HAS_CREDENTIAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasCredentialPropertyIri> for &str {
	fn eq(&self, other: &HasCredentialPropertyIri) -> bool {
		*self == HAS_CREDENTIAL_PROPERTY_IRI_HTTP || *self == HAS_CREDENTIAL_PROPERTY_IRI_HTTPS
	}
}
pub struct HasCredentialPropertyIriOrLabel;
impl PartialEq<&str> for HasCredentialPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasCredentialPropertyIri || *other == HAS_CREDENTIAL_PROPERTY_LABEL
	}
}
impl PartialEq<HasCredentialPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasCredentialPropertyIriOrLabel) -> bool {
		*self == HasCredentialPropertyIri || *self == HAS_CREDENTIAL_PROPERTY_LABEL
	}
}
