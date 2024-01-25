/// <https://schema.org/PeopleAudience>
pub const PEOPLE_AUDIENCE_IRI_HTTP: &str = "http://schema.org/PeopleAudience";
/// <https://schema.org/PeopleAudience>
pub const PEOPLE_AUDIENCE_IRI_HTTPS: &str = "https://schema.org/PeopleAudience";
/// <https://schema.org/PeopleAudience>
pub const PEOPLE_AUDIENCE_LABEL: &str = "PeopleAudience";
pub struct PeopleAudienceIri;
impl PartialEq<&str> for PeopleAudienceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PEOPLE_AUDIENCE_IRI_HTTP || *other == PEOPLE_AUDIENCE_IRI_HTTPS
	}
}
impl PartialEq<PeopleAudienceIri> for &str {
	fn eq(&self, other: &PeopleAudienceIri) -> bool {
		*self == PEOPLE_AUDIENCE_IRI_HTTP || *self == PEOPLE_AUDIENCE_IRI_HTTPS
	}
}
pub struct PeopleAudienceIriOrLabel;
impl PartialEq<&str> for PeopleAudienceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PeopleAudienceIri || *other == PEOPLE_AUDIENCE_LABEL
	}
}
impl PartialEq<PeopleAudienceIriOrLabel> for &str {
	fn eq(&self, other: &PeopleAudienceIriOrLabel) -> bool {
		*self == PeopleAudienceIri || *self == PEOPLE_AUDIENCE_LABEL
	}
}
