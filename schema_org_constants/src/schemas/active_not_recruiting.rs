/// <https://schema.org/ActiveNotRecruiting>
pub const ACTIVE_NOT_RECRUITING_IRI_HTTP: &str = "http://schema.org/ActiveNotRecruiting";
/// <https://schema.org/ActiveNotRecruiting>
pub const ACTIVE_NOT_RECRUITING_IRI_HTTPS: &str = "https://schema.org/ActiveNotRecruiting";
/// <https://schema.org/ActiveNotRecruiting>
pub const ACTIVE_NOT_RECRUITING_LABEL: &str = "ActiveNotRecruiting";
pub struct ActiveNotRecruitingIri;
impl PartialEq<&str> for ActiveNotRecruitingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTIVE_NOT_RECRUITING_IRI_HTTP || *other == ACTIVE_NOT_RECRUITING_IRI_HTTPS
	}
}
impl PartialEq<ActiveNotRecruitingIri> for &str {
	fn eq(&self, other: &ActiveNotRecruitingIri) -> bool {
		*self == ACTIVE_NOT_RECRUITING_IRI_HTTP || *self == ACTIVE_NOT_RECRUITING_IRI_HTTPS
	}
}
pub struct ActiveNotRecruitingIriOrLabel;
impl PartialEq<&str> for ActiveNotRecruitingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActiveNotRecruitingIri || *other == ACTIVE_NOT_RECRUITING_LABEL
	}
}
impl PartialEq<ActiveNotRecruitingIriOrLabel> for &str {
	fn eq(&self, other: &ActiveNotRecruitingIriOrLabel) -> bool {
		*self == ActiveNotRecruitingIri || *self == ACTIVE_NOT_RECRUITING_LABEL
	}
}
