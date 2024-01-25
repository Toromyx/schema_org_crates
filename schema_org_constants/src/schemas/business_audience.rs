/// <https://schema.org/BusinessAudience>
pub const BUSINESS_AUDIENCE_IRI_HTTP: &str = "http://schema.org/BusinessAudience";
/// <https://schema.org/BusinessAudience>
pub const BUSINESS_AUDIENCE_IRI_HTTPS: &str = "https://schema.org/BusinessAudience";
/// <https://schema.org/BusinessAudience>
pub const BUSINESS_AUDIENCE_LABEL: &str = "BusinessAudience";
pub struct BusinessAudienceIri;
impl PartialEq<&str> for BusinessAudienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BUSINESS_AUDIENCE_IRI_HTTP || *other == BUSINESS_AUDIENCE_IRI_HTTPS
	}
}
impl PartialEq<BusinessAudienceIri> for &str {
	fn eq(&self, other: &BusinessAudienceIri) -> bool {
		*self == BUSINESS_AUDIENCE_IRI_HTTP || *self == BUSINESS_AUDIENCE_IRI_HTTPS
	}
}
pub struct BusinessAudienceIriOrLabel;
impl PartialEq<&str> for BusinessAudienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BusinessAudienceIri || *other == BUSINESS_AUDIENCE_LABEL
	}
}
impl PartialEq<BusinessAudienceIriOrLabel> for &str {
	fn eq(&self, other: &BusinessAudienceIriOrLabel) -> bool {
		*self == BusinessAudienceIri || *self == BUSINESS_AUDIENCE_LABEL
	}
}
