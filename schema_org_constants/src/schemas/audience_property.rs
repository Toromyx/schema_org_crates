/// <https://schema.org/audience>
pub const AUDIENCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/audience";
/// <https://schema.org/audience>
pub const AUDIENCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/audience";
/// <https://schema.org/audience>
pub const AUDIENCE_PROPERTY_LABEL: &str = "audience";
pub struct AudiencePropertyIri;
impl PartialEq<&str> for AudiencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIENCE_PROPERTY_IRI_HTTP || *other == AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AudiencePropertyIri> for &str {
	fn eq(&self, other: &AudiencePropertyIri) -> bool {
		*self == AUDIENCE_PROPERTY_IRI_HTTP || *self == AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct AudiencePropertyIriOrLabel;
impl PartialEq<&str> for AudiencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudiencePropertyIri || *other == AUDIENCE_PROPERTY_LABEL
	}
}
impl PartialEq<AudiencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AudiencePropertyIriOrLabel) -> bool {
		*self == AudiencePropertyIri || *self == AUDIENCE_PROPERTY_LABEL
	}
}
