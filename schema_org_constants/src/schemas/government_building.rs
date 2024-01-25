/// <https://schema.org/GovernmentBuilding>
pub const GOVERNMENT_BUILDING_IRI_HTTP: &str = "http://schema.org/GovernmentBuilding";
/// <https://schema.org/GovernmentBuilding>
pub const GOVERNMENT_BUILDING_IRI_HTTPS: &str = "https://schema.org/GovernmentBuilding";
/// <https://schema.org/GovernmentBuilding>
pub const GOVERNMENT_BUILDING_LABEL: &str = "GovernmentBuilding";
pub struct GovernmentBuildingIri;
impl PartialEq<&str> for GovernmentBuildingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GOVERNMENT_BUILDING_IRI_HTTP || *other == GOVERNMENT_BUILDING_IRI_HTTPS
	}
}
impl PartialEq<GovernmentBuildingIri> for &str {
	fn eq(&self, other: &GovernmentBuildingIri) -> bool {
		*self == GOVERNMENT_BUILDING_IRI_HTTP || *self == GOVERNMENT_BUILDING_IRI_HTTPS
	}
}
pub struct GovernmentBuildingIriOrLabel;
impl PartialEq<&str> for GovernmentBuildingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GovernmentBuildingIri || *other == GOVERNMENT_BUILDING_LABEL
	}
}
impl PartialEq<GovernmentBuildingIriOrLabel> for &str {
	fn eq(&self, other: &GovernmentBuildingIriOrLabel) -> bool {
		*self == GovernmentBuildingIri || *self == GOVERNMENT_BUILDING_LABEL
	}
}
