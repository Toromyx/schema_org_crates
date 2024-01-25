/// <https://schema.org/bloodSupply>
pub const BLOOD_SUPPLY_PROPERTY_IRI_HTTP: &str = "http://schema.org/bloodSupply";
/// <https://schema.org/bloodSupply>
pub const BLOOD_SUPPLY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bloodSupply";
/// <https://schema.org/bloodSupply>
pub const BLOOD_SUPPLY_PROPERTY_LABEL: &str = "bloodSupply";
pub struct BloodSupplyPropertyIri;
impl PartialEq<&str> for BloodSupplyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BLOOD_SUPPLY_PROPERTY_IRI_HTTP || *other == BLOOD_SUPPLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BloodSupplyPropertyIri> for &str {
	fn eq(&self, other: &BloodSupplyPropertyIri) -> bool {
		*self == BLOOD_SUPPLY_PROPERTY_IRI_HTTP || *self == BLOOD_SUPPLY_PROPERTY_IRI_HTTPS
	}
}
pub struct BloodSupplyPropertyIriOrLabel;
impl PartialEq<&str> for BloodSupplyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BloodSupplyPropertyIri || *other == BLOOD_SUPPLY_PROPERTY_LABEL
	}
}
impl PartialEq<BloodSupplyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BloodSupplyPropertyIriOrLabel) -> bool {
		*self == BloodSupplyPropertyIri || *self == BLOOD_SUPPLY_PROPERTY_LABEL
	}
}
