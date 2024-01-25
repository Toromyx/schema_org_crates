/// <https://schema.org/publicAccess>
pub const PUBLIC_ACCESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/publicAccess";
/// <https://schema.org/publicAccess>
pub const PUBLIC_ACCESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/publicAccess";
/// <https://schema.org/publicAccess>
pub const PUBLIC_ACCESS_PROPERTY_LABEL: &str = "publicAccess";
pub struct PublicAccessPropertyIri;
impl PartialEq<&str> for PublicAccessPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLIC_ACCESS_PROPERTY_IRI_HTTP || *other == PUBLIC_ACCESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublicAccessPropertyIri> for &str {
	fn eq(&self, other: &PublicAccessPropertyIri) -> bool {
		*self == PUBLIC_ACCESS_PROPERTY_IRI_HTTP || *self == PUBLIC_ACCESS_PROPERTY_IRI_HTTPS
	}
}
pub struct PublicAccessPropertyIriOrLabel;
impl PartialEq<&str> for PublicAccessPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicAccessPropertyIri || *other == PUBLIC_ACCESS_PROPERTY_LABEL
	}
}
impl PartialEq<PublicAccessPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublicAccessPropertyIriOrLabel) -> bool {
		*self == PublicAccessPropertyIri || *self == PUBLIC_ACCESS_PROPERTY_LABEL
	}
}
