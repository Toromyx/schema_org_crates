/// <https://schema.org/EnergyStarCertified>
pub const ENERGY_STAR_CERTIFIED_IRI_HTTP: &str = "http://schema.org/EnergyStarCertified";
/// <https://schema.org/EnergyStarCertified>
pub const ENERGY_STAR_CERTIFIED_IRI_HTTPS: &str = "https://schema.org/EnergyStarCertified";
/// <https://schema.org/EnergyStarCertified>
pub const ENERGY_STAR_CERTIFIED_LABEL: &str = "EnergyStarCertified";
pub struct EnergyStarCertifiedIri;
impl PartialEq<&str> for EnergyStarCertifiedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_STAR_CERTIFIED_IRI_HTTP || *other == ENERGY_STAR_CERTIFIED_IRI_HTTPS
	}
}
impl PartialEq<EnergyStarCertifiedIri> for &str {
	fn eq(&self, other: &EnergyStarCertifiedIri) -> bool {
		*self == ENERGY_STAR_CERTIFIED_IRI_HTTP || *self == ENERGY_STAR_CERTIFIED_IRI_HTTPS
	}
}
pub struct EnergyStarCertifiedIriOrLabel;
impl PartialEq<&str> for EnergyStarCertifiedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyStarCertifiedIri || *other == ENERGY_STAR_CERTIFIED_LABEL
	}
}
impl PartialEq<EnergyStarCertifiedIriOrLabel> for &str {
	fn eq(&self, other: &EnergyStarCertifiedIriOrLabel) -> bool {
		*self == EnergyStarCertifiedIri || *self == ENERGY_STAR_CERTIFIED_LABEL
	}
}
