/// <https://schema.org/NotYetRecruiting>
pub const NOT_YET_RECRUITING_IRI_HTTP: &str = "http://schema.org/NotYetRecruiting";
/// <https://schema.org/NotYetRecruiting>
pub const NOT_YET_RECRUITING_IRI_HTTPS: &str = "https://schema.org/NotYetRecruiting";
/// <https://schema.org/NotYetRecruiting>
pub const NOT_YET_RECRUITING_LABEL: &str = "NotYetRecruiting";
pub struct NotYetRecruitingIri;
impl PartialEq<&str> for NotYetRecruitingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NOT_YET_RECRUITING_IRI_HTTP || *other == NOT_YET_RECRUITING_IRI_HTTPS
	}
}
impl PartialEq<NotYetRecruitingIri> for &str {
	fn eq(&self, other: &NotYetRecruitingIri) -> bool {
		*self == NOT_YET_RECRUITING_IRI_HTTP || *self == NOT_YET_RECRUITING_IRI_HTTPS
	}
}
pub struct NotYetRecruitingIriOrLabel;
impl PartialEq<&str> for NotYetRecruitingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NotYetRecruitingIri || *other == NOT_YET_RECRUITING_LABEL
	}
}
impl PartialEq<NotYetRecruitingIriOrLabel> for &str {
	fn eq(&self, other: &NotYetRecruitingIriOrLabel) -> bool {
		*self == NotYetRecruitingIri || *self == NOT_YET_RECRUITING_LABEL
	}
}
