/// <https://schema.org/LegislativeBuilding>
pub const LEGISLATIVE_BUILDING_IRI_HTTP: &str = "http://schema.org/LegislativeBuilding";
/// <https://schema.org/LegislativeBuilding>
pub const LEGISLATIVE_BUILDING_IRI_HTTPS: &str = "https://schema.org/LegislativeBuilding";
/// <https://schema.org/LegislativeBuilding>
pub const LEGISLATIVE_BUILDING_LABEL: &str = "LegislativeBuilding";
pub struct LegislativeBuildingIri;
impl PartialEq<&str> for LegislativeBuildingIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATIVE_BUILDING_IRI_HTTP || *other == LEGISLATIVE_BUILDING_IRI_HTTPS
	}
}
impl PartialEq<LegislativeBuildingIri> for &str {
	fn eq(&self, other: &LegislativeBuildingIri) -> bool {
		*self == LEGISLATIVE_BUILDING_IRI_HTTP || *self == LEGISLATIVE_BUILDING_IRI_HTTPS
	}
}
pub struct LegislativeBuildingIriOrLabel;
impl PartialEq<&str> for LegislativeBuildingIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislativeBuildingIri || *other == LEGISLATIVE_BUILDING_LABEL
	}
}
impl PartialEq<LegislativeBuildingIriOrLabel> for &str {
	fn eq(&self, other: &LegislativeBuildingIriOrLabel) -> bool {
		*self == LegislativeBuildingIri || *self == LEGISLATIVE_BUILDING_LABEL
	}
}
