/// <https://schema.org/supplyTo>
pub const SUPPLY_TO_PROPERTY_IRI_HTTP: &str = "http://schema.org/supplyTo";
/// <https://schema.org/supplyTo>
pub const SUPPLY_TO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/supplyTo";
/// <https://schema.org/supplyTo>
pub const SUPPLY_TO_PROPERTY_LABEL: &str = "supplyTo";
pub struct SupplyToPropertyIri;
impl PartialEq<&str> for SupplyToPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUPPLY_TO_PROPERTY_IRI_HTTP || *other == SUPPLY_TO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SupplyToPropertyIri> for &str {
	fn eq(&self, other: &SupplyToPropertyIri) -> bool {
		*self == SUPPLY_TO_PROPERTY_IRI_HTTP || *self == SUPPLY_TO_PROPERTY_IRI_HTTPS
	}
}
pub struct SupplyToPropertyIriOrLabel;
impl PartialEq<&str> for SupplyToPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SupplyToPropertyIri || *other == SUPPLY_TO_PROPERTY_LABEL
	}
}
impl PartialEq<SupplyToPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SupplyToPropertyIriOrLabel) -> bool {
		*self == SupplyToPropertyIri || *self == SUPPLY_TO_PROPERTY_LABEL
	}
}
