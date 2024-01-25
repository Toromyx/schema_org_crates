/// <https://schema.org/boardingGroup>
pub const BOARDING_GROUP_PROPERTY_IRI_HTTP: &str = "http://schema.org/boardingGroup";
/// <https://schema.org/boardingGroup>
pub const BOARDING_GROUP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/boardingGroup";
/// <https://schema.org/boardingGroup>
pub const BOARDING_GROUP_PROPERTY_LABEL: &str = "boardingGroup";
pub struct BoardingGroupPropertyIri;
impl PartialEq<&str> for BoardingGroupPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOARDING_GROUP_PROPERTY_IRI_HTTP || *other == BOARDING_GROUP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BoardingGroupPropertyIri> for &str {
	fn eq(&self, other: &BoardingGroupPropertyIri) -> bool {
		*self == BOARDING_GROUP_PROPERTY_IRI_HTTP || *self == BOARDING_GROUP_PROPERTY_IRI_HTTPS
	}
}
pub struct BoardingGroupPropertyIriOrLabel;
impl PartialEq<&str> for BoardingGroupPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoardingGroupPropertyIri || *other == BOARDING_GROUP_PROPERTY_LABEL
	}
}
impl PartialEq<BoardingGroupPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BoardingGroupPropertyIriOrLabel) -> bool {
		*self == BoardingGroupPropertyIri || *self == BOARDING_GROUP_PROPERTY_LABEL
	}
}
