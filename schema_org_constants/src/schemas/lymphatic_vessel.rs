/// <https://schema.org/LymphaticVessel>
pub const LYMPHATIC_VESSEL_IRI_HTTP: &str = "http://schema.org/LymphaticVessel";
/// <https://schema.org/LymphaticVessel>
pub const LYMPHATIC_VESSEL_IRI_HTTPS: &str = "https://schema.org/LymphaticVessel";
/// <https://schema.org/LymphaticVessel>
pub const LYMPHATIC_VESSEL_LABEL: &str = "LymphaticVessel";
pub struct LymphaticVesselIri;
impl PartialEq<&str> for LymphaticVesselIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LYMPHATIC_VESSEL_IRI_HTTP || *other == LYMPHATIC_VESSEL_IRI_HTTPS
	}
}
impl PartialEq<LymphaticVesselIri> for &str {
	fn eq(&self, other: &LymphaticVesselIri) -> bool {
		*self == LYMPHATIC_VESSEL_IRI_HTTP || *self == LYMPHATIC_VESSEL_IRI_HTTPS
	}
}
pub struct LymphaticVesselIriOrLabel;
impl PartialEq<&str> for LymphaticVesselIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LymphaticVesselIri || *other == LYMPHATIC_VESSEL_LABEL
	}
}
impl PartialEq<LymphaticVesselIriOrLabel> for &str {
	fn eq(&self, other: &LymphaticVesselIriOrLabel) -> bool {
		*self == LymphaticVesselIri || *self == LYMPHATIC_VESSEL_LABEL
	}
}
