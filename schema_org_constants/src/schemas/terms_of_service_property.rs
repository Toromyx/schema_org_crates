/// <https://schema.org/termsOfService>
pub const TERMS_OF_SERVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/termsOfService";
/// <https://schema.org/termsOfService>
pub const TERMS_OF_SERVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/termsOfService";
/// <https://schema.org/termsOfService>
pub const TERMS_OF_SERVICE_PROPERTY_LABEL: &str = "termsOfService";
pub struct TermsOfServicePropertyIri;
impl PartialEq<&str> for TermsOfServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TERMS_OF_SERVICE_PROPERTY_IRI_HTTP
			|| *other == TERMS_OF_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TermsOfServicePropertyIri> for &str {
	fn eq(&self, other: &TermsOfServicePropertyIri) -> bool {
		*self == TERMS_OF_SERVICE_PROPERTY_IRI_HTTP || *self == TERMS_OF_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct TermsOfServicePropertyIriOrLabel;
impl PartialEq<&str> for TermsOfServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TermsOfServicePropertyIri || *other == TERMS_OF_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<TermsOfServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TermsOfServicePropertyIriOrLabel) -> bool {
		*self == TermsOfServicePropertyIri || *self == TERMS_OF_SERVICE_PROPERTY_LABEL
	}
}
