/// <https://schema.org/audienceType>
pub const AUDIENCE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/audienceType";
/// <https://schema.org/audienceType>
pub const AUDIENCE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/audienceType";
/// <https://schema.org/audienceType>
pub const AUDIENCE_TYPE_PROPERTY_LABEL: &str = "audienceType";
pub struct AudienceTypePropertyIri;
impl PartialEq<&str> for AudienceTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUDIENCE_TYPE_PROPERTY_IRI_HTTP || *other == AUDIENCE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AudienceTypePropertyIri> for &str {
	fn eq(&self, other: &AudienceTypePropertyIri) -> bool {
		*self == AUDIENCE_TYPE_PROPERTY_IRI_HTTP || *self == AUDIENCE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct AudienceTypePropertyIriOrLabel;
impl PartialEq<&str> for AudienceTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AudienceTypePropertyIri || *other == AUDIENCE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<AudienceTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AudienceTypePropertyIriOrLabel) -> bool {
		*self == AudienceTypePropertyIri || *self == AUDIENCE_TYPE_PROPERTY_LABEL
	}
}
