/// <https://schema.org/Audience>
pub const AUDIENCE_IRI_HTTP: &str = "http://schema.org/Audience";
/// <https://schema.org/Audience>
pub const AUDIENCE_IRI_HTTPS: &str = "https://schema.org/Audience";
/// <https://schema.org/Audience>
pub const AUDIENCE_LABEL: &str = "Audience";
pub struct AudienceIri;
impl PartialEq<&str> for AudienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIENCE_IRI_HTTP || *other == AUDIENCE_IRI_HTTPS
	}
}
impl PartialEq<AudienceIri> for &str {
	fn eq(&self, other: &AudienceIri) -> bool {
		*self == AUDIENCE_IRI_HTTP || *self == AUDIENCE_IRI_HTTPS
	}
}
pub struct AudienceIriOrLabel;
impl PartialEq<&str> for AudienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudienceIri || *other == AUDIENCE_LABEL
	}
}
impl PartialEq<AudienceIriOrLabel> for &str {
	fn eq(&self, other: &AudienceIriOrLabel) -> bool {
		*self == AudienceIri || *self == AUDIENCE_LABEL
	}
}
