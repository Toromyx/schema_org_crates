/// <https://schema.org/colleague>
pub const COLLEAGUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/colleague";
/// <https://schema.org/colleague>
pub const COLLEAGUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/colleague";
/// <https://schema.org/colleague>
pub const COLLEAGUE_PROPERTY_LABEL: &str = "colleague";
pub struct ColleaguePropertyIri;
impl PartialEq<&str> for ColleaguePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COLLEAGUE_PROPERTY_IRI_HTTP || *other == COLLEAGUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ColleaguePropertyIri> for &str {
	fn eq(&self, other: &ColleaguePropertyIri) -> bool {
		*self == COLLEAGUE_PROPERTY_IRI_HTTP || *self == COLLEAGUE_PROPERTY_IRI_HTTPS
	}
}
pub struct ColleaguePropertyIriOrLabel;
impl PartialEq<&str> for ColleaguePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ColleaguePropertyIri || *other == COLLEAGUE_PROPERTY_LABEL
	}
}
impl PartialEq<ColleaguePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ColleaguePropertyIriOrLabel) -> bool {
		*self == ColleaguePropertyIri || *self == COLLEAGUE_PROPERTY_LABEL
	}
}
