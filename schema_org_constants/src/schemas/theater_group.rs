/// <https://schema.org/TheaterGroup>
pub const THEATER_GROUP_IRI_HTTP: &str = "http://schema.org/TheaterGroup";
/// <https://schema.org/TheaterGroup>
pub const THEATER_GROUP_IRI_HTTPS: &str = "https://schema.org/TheaterGroup";
/// <https://schema.org/TheaterGroup>
pub const THEATER_GROUP_LABEL: &str = "TheaterGroup";
pub struct TheaterGroupIri;
impl PartialEq<&str> for TheaterGroupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THEATER_GROUP_IRI_HTTP || *other == THEATER_GROUP_IRI_HTTPS
	}
}
impl PartialEq<TheaterGroupIri> for &str {
	fn eq(&self, other: &TheaterGroupIri) -> bool {
		*self == THEATER_GROUP_IRI_HTTP || *self == THEATER_GROUP_IRI_HTTPS
	}
}
pub struct TheaterGroupIriOrLabel;
impl PartialEq<&str> for TheaterGroupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TheaterGroupIri || *other == THEATER_GROUP_LABEL
	}
}
impl PartialEq<TheaterGroupIriOrLabel> for &str {
	fn eq(&self, other: &TheaterGroupIriOrLabel) -> bool {
		*self == TheaterGroupIri || *self == THEATER_GROUP_LABEL
	}
}
