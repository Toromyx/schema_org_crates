/// <https://schema.org/HowToSupply>
pub const HOW_TO_SUPPLY_IRI_HTTP: &str = "http://schema.org/HowToSupply";
/// <https://schema.org/HowToSupply>
pub const HOW_TO_SUPPLY_IRI_HTTPS: &str = "https://schema.org/HowToSupply";
/// <https://schema.org/HowToSupply>
pub const HOW_TO_SUPPLY_LABEL: &str = "HowToSupply";
pub struct HowToSupplyIri;
impl PartialEq<&str> for HowToSupplyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOW_TO_SUPPLY_IRI_HTTP || *other == HOW_TO_SUPPLY_IRI_HTTPS
	}
}
impl PartialEq<HowToSupplyIri> for &str {
	fn eq(&self, other: &HowToSupplyIri) -> bool {
		*self == HOW_TO_SUPPLY_IRI_HTTP || *self == HOW_TO_SUPPLY_IRI_HTTPS
	}
}
pub struct HowToSupplyIriOrLabel;
impl PartialEq<&str> for HowToSupplyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HowToSupplyIri || *other == HOW_TO_SUPPLY_LABEL
	}
}
impl PartialEq<HowToSupplyIriOrLabel> for &str {
	fn eq(&self, other: &HowToSupplyIriOrLabel) -> bool {
		*self == HowToSupplyIri || *self == HOW_TO_SUPPLY_LABEL
	}
}
