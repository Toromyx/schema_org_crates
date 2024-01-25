/// <https://schema.org/Vessel>
pub const VESSEL_IRI_HTTP: &str = "http://schema.org/Vessel";
/// <https://schema.org/Vessel>
pub const VESSEL_IRI_HTTPS: &str = "https://schema.org/Vessel";
/// <https://schema.org/Vessel>
pub const VESSEL_LABEL: &str = "Vessel";
pub struct VesselIri;
impl PartialEq<&str> for VesselIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VESSEL_IRI_HTTP || *other == VESSEL_IRI_HTTPS
	}
}
impl PartialEq<VesselIri> for &str {
	fn eq(&self, other: &VesselIri) -> bool {
		*self == VESSEL_IRI_HTTP || *self == VESSEL_IRI_HTTPS
	}
}
pub struct VesselIriOrLabel;
impl PartialEq<&str> for VesselIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VesselIri || *other == VESSEL_LABEL
	}
}
impl PartialEq<VesselIriOrLabel> for &str {
	fn eq(&self, other: &VesselIriOrLabel) -> bool {
		*self == VesselIri || *self == VESSEL_LABEL
	}
}
