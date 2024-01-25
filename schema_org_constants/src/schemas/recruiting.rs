/// <https://schema.org/Recruiting>
pub const RECRUITING_IRI_HTTP: &str = "http://schema.org/Recruiting";
/// <https://schema.org/Recruiting>
pub const RECRUITING_IRI_HTTPS: &str = "https://schema.org/Recruiting";
/// <https://schema.org/Recruiting>
pub const RECRUITING_LABEL: &str = "Recruiting";
pub struct RecruitingIri;
impl PartialEq<&str> for RecruitingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECRUITING_IRI_HTTP || *other == RECRUITING_IRI_HTTPS
	}
}
impl PartialEq<RecruitingIri> for &str {
	fn eq(&self, other: &RecruitingIri) -> bool {
		*self == RECRUITING_IRI_HTTP || *self == RECRUITING_IRI_HTTPS
	}
}
pub struct RecruitingIriOrLabel;
impl PartialEq<&str> for RecruitingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecruitingIri || *other == RECRUITING_LABEL
	}
}
impl PartialEq<RecruitingIriOrLabel> for &str {
	fn eq(&self, other: &RecruitingIriOrLabel) -> bool {
		*self == RecruitingIri || *self == RECRUITING_LABEL
	}
}
