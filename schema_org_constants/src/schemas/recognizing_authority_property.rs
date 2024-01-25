/// <https://schema.org/recognizingAuthority>
pub const RECOGNIZING_AUTHORITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/recognizingAuthority";
/// <https://schema.org/recognizingAuthority>
pub const RECOGNIZING_AUTHORITY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/recognizingAuthority";
/// <https://schema.org/recognizingAuthority>
pub const RECOGNIZING_AUTHORITY_PROPERTY_LABEL: &str = "recognizingAuthority";
pub struct RecognizingAuthorityPropertyIri;
impl PartialEq<&str> for RecognizingAuthorityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOGNIZING_AUTHORITY_PROPERTY_IRI_HTTP
			|| *other == RECOGNIZING_AUTHORITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecognizingAuthorityPropertyIri> for &str {
	fn eq(&self, other: &RecognizingAuthorityPropertyIri) -> bool {
		*self == RECOGNIZING_AUTHORITY_PROPERTY_IRI_HTTP
			|| *self == RECOGNIZING_AUTHORITY_PROPERTY_IRI_HTTPS
	}
}
pub struct RecognizingAuthorityPropertyIriOrLabel;
impl PartialEq<&str> for RecognizingAuthorityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecognizingAuthorityPropertyIri || *other == RECOGNIZING_AUTHORITY_PROPERTY_LABEL
	}
}
impl PartialEq<RecognizingAuthorityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecognizingAuthorityPropertyIriOrLabel) -> bool {
		*self == RecognizingAuthorityPropertyIri || *self == RECOGNIZING_AUTHORITY_PROPERTY_LABEL
	}
}
