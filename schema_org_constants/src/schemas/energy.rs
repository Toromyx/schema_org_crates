/// <https://schema.org/Energy>
pub const ENERGY_IRI_HTTP: &str = "http://schema.org/Energy";
/// <https://schema.org/Energy>
pub const ENERGY_IRI_HTTPS: &str = "https://schema.org/Energy";
/// <https://schema.org/Energy>
pub const ENERGY_LABEL: &str = "Energy";
pub struct EnergyIri;
impl PartialEq<&str> for EnergyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENERGY_IRI_HTTP || *other == ENERGY_IRI_HTTPS
	}
}
impl PartialEq<EnergyIri> for &str {
	fn eq(&self, other: &EnergyIri) -> bool {
		*self == ENERGY_IRI_HTTP || *self == ENERGY_IRI_HTTPS
	}
}
pub struct EnergyIriOrLabel;
impl PartialEq<&str> for EnergyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EnergyIri || *other == ENERGY_LABEL
	}
}
impl PartialEq<EnergyIriOrLabel> for &str {
	fn eq(&self, other: &EnergyIriOrLabel) -> bool {
		*self == EnergyIri || *self == ENERGY_LABEL
	}
}
