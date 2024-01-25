/// <https://schema.org/permitAudience>
pub const PERMIT_AUDIENCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/permitAudience";
/// <https://schema.org/permitAudience>
pub const PERMIT_AUDIENCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/permitAudience";
/// <https://schema.org/permitAudience>
pub const PERMIT_AUDIENCE_PROPERTY_LABEL: &str = "permitAudience";
pub struct PermitAudiencePropertyIri;
impl PartialEq<&str> for PermitAudiencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERMIT_AUDIENCE_PROPERTY_IRI_HTTP || *other == PERMIT_AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PermitAudiencePropertyIri> for &str {
	fn eq(&self, other: &PermitAudiencePropertyIri) -> bool {
		*self == PERMIT_AUDIENCE_PROPERTY_IRI_HTTP || *self == PERMIT_AUDIENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct PermitAudiencePropertyIriOrLabel;
impl PartialEq<&str> for PermitAudiencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PermitAudiencePropertyIri || *other == PERMIT_AUDIENCE_PROPERTY_LABEL
	}
}
impl PartialEq<PermitAudiencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &PermitAudiencePropertyIriOrLabel) -> bool {
		*self == PermitAudiencePropertyIri || *self == PERMIT_AUDIENCE_PROPERTY_LABEL
	}
}
