/// <https://schema.org/supply>
pub const SUPPLY_PROPERTY_IRI_HTTP: &str = "http://schema.org/supply";
/// <https://schema.org/supply>
pub const SUPPLY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/supply";
/// <https://schema.org/supply>
pub const SUPPLY_PROPERTY_LABEL: &str = "supply";
pub struct SupplyPropertyIri;
impl PartialEq<&str> for SupplyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUPPLY_PROPERTY_IRI_HTTP || *other == SUPPLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SupplyPropertyIri> for &str {
	fn eq(&self, other: &SupplyPropertyIri) -> bool {
		*self == SUPPLY_PROPERTY_IRI_HTTP || *self == SUPPLY_PROPERTY_IRI_HTTPS
	}
}
pub struct SupplyPropertyIriOrLabel;
impl PartialEq<&str> for SupplyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SupplyPropertyIri || *other == SUPPLY_PROPERTY_LABEL
	}
}
impl PartialEq<SupplyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SupplyPropertyIriOrLabel) -> bool {
		*self == SupplyPropertyIri || *self == SUPPLY_PROPERTY_LABEL
	}
}
