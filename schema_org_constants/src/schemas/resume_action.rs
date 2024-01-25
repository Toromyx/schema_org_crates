/// <https://schema.org/ResumeAction>
pub const RESUME_ACTION_IRI_HTTP: &str = "http://schema.org/ResumeAction";
/// <https://schema.org/ResumeAction>
pub const RESUME_ACTION_IRI_HTTPS: &str = "https://schema.org/ResumeAction";
/// <https://schema.org/ResumeAction>
pub const RESUME_ACTION_LABEL: &str = "ResumeAction";
pub struct ResumeActionIri;
impl PartialEq<&str> for ResumeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESUME_ACTION_IRI_HTTP || *other == RESUME_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ResumeActionIri> for &str {
	fn eq(&self, other: &ResumeActionIri) -> bool {
		*self == RESUME_ACTION_IRI_HTTP || *self == RESUME_ACTION_IRI_HTTPS
	}
}
pub struct ResumeActionIriOrLabel;
impl PartialEq<&str> for ResumeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResumeActionIri || *other == RESUME_ACTION_LABEL
	}
}
impl PartialEq<ResumeActionIriOrLabel> for &str {
	fn eq(&self, other: &ResumeActionIriOrLabel) -> bool {
		*self == ResumeActionIri || *self == RESUME_ACTION_LABEL
	}
}
