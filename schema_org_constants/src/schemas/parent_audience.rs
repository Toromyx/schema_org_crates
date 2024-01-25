/// <https://schema.org/ParentAudience>
pub const PARENT_AUDIENCE_IRI_HTTP: &str = "http://schema.org/ParentAudience";
/// <https://schema.org/ParentAudience>
pub const PARENT_AUDIENCE_IRI_HTTPS: &str = "https://schema.org/ParentAudience";
/// <https://schema.org/ParentAudience>
pub const PARENT_AUDIENCE_LABEL: &str = "ParentAudience";
pub struct ParentAudienceIri;
impl PartialEq<&str> for ParentAudienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARENT_AUDIENCE_IRI_HTTP || *other == PARENT_AUDIENCE_IRI_HTTPS
	}
}
impl PartialEq<ParentAudienceIri> for &str {
	fn eq(&self, other: &ParentAudienceIri) -> bool {
		*self == PARENT_AUDIENCE_IRI_HTTP || *self == PARENT_AUDIENCE_IRI_HTTPS
	}
}
pub struct ParentAudienceIriOrLabel;
impl PartialEq<&str> for ParentAudienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParentAudienceIri || *other == PARENT_AUDIENCE_LABEL
	}
}
impl PartialEq<ParentAudienceIriOrLabel> for &str {
	fn eq(&self, other: &ParentAudienceIriOrLabel) -> bool {
		*self == ParentAudienceIri || *self == PARENT_AUDIENCE_LABEL
	}
}
