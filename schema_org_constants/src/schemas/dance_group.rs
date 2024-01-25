/// <https://schema.org/DanceGroup>
pub const DANCE_GROUP_IRI_HTTP: &str = "http://schema.org/DanceGroup";
/// <https://schema.org/DanceGroup>
pub const DANCE_GROUP_IRI_HTTPS: &str = "https://schema.org/DanceGroup";
/// <https://schema.org/DanceGroup>
pub const DANCE_GROUP_LABEL: &str = "DanceGroup";
pub struct DanceGroupIri;
impl PartialEq<&str> for DanceGroupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DANCE_GROUP_IRI_HTTP || *other == DANCE_GROUP_IRI_HTTPS
	}
}
impl PartialEq<DanceGroupIri> for &str {
	fn eq(&self, other: &DanceGroupIri) -> bool {
		*self == DANCE_GROUP_IRI_HTTP || *self == DANCE_GROUP_IRI_HTTPS
	}
}
pub struct DanceGroupIriOrLabel;
impl PartialEq<&str> for DanceGroupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DanceGroupIri || *other == DANCE_GROUP_LABEL
	}
}
impl PartialEq<DanceGroupIriOrLabel> for &str {
	fn eq(&self, other: &DanceGroupIriOrLabel) -> bool {
		*self == DanceGroupIri || *self == DANCE_GROUP_LABEL
	}
}
