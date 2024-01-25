/// <https://schema.org/EducationalAudience>
pub const EDUCATIONAL_AUDIENCE_IRI_HTTP: &str = "http://schema.org/EducationalAudience";
/// <https://schema.org/EducationalAudience>
pub const EDUCATIONAL_AUDIENCE_IRI_HTTPS: &str = "https://schema.org/EducationalAudience";
/// <https://schema.org/EducationalAudience>
pub const EDUCATIONAL_AUDIENCE_LABEL: &str = "EducationalAudience";
pub struct EducationalAudienceIri;
impl PartialEq<&str> for EducationalAudienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDUCATIONAL_AUDIENCE_IRI_HTTP || *other == EDUCATIONAL_AUDIENCE_IRI_HTTPS
	}
}
impl PartialEq<EducationalAudienceIri> for &str {
	fn eq(&self, other: &EducationalAudienceIri) -> bool {
		*self == EDUCATIONAL_AUDIENCE_IRI_HTTP || *self == EDUCATIONAL_AUDIENCE_IRI_HTTPS
	}
}
pub struct EducationalAudienceIriOrLabel;
impl PartialEq<&str> for EducationalAudienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EducationalAudienceIri || *other == EDUCATIONAL_AUDIENCE_LABEL
	}
}
impl PartialEq<EducationalAudienceIriOrLabel> for &str {
	fn eq(&self, other: &EducationalAudienceIriOrLabel) -> bool {
		*self == EducationalAudienceIri || *self == EDUCATIONAL_AUDIENCE_LABEL
	}
}
