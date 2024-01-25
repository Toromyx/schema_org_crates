/// <https://schema.org/Grant>
pub const GRANT_IRI_HTTP: &str = "http://schema.org/Grant";
/// <https://schema.org/Grant>
pub const GRANT_IRI_HTTPS: &str = "https://schema.org/Grant";
/// <https://schema.org/Grant>
pub const GRANT_LABEL: &str = "Grant";
pub struct GrantIri;
impl PartialEq<&str> for GrantIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GRANT_IRI_HTTP || *other == GRANT_IRI_HTTPS
	}
}
impl PartialEq<GrantIri> for &str {
	fn eq(&self, other: &GrantIri) -> bool {
		*self == GRANT_IRI_HTTP || *self == GRANT_IRI_HTTPS
	}
}
pub struct GrantIriOrLabel;
impl PartialEq<&str> for GrantIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GrantIri || *other == GRANT_LABEL
	}
}
impl PartialEq<GrantIriOrLabel> for &str {
	fn eq(&self, other: &GrantIriOrLabel) -> bool {
		*self == GrantIri || *self == GRANT_LABEL
	}
}
