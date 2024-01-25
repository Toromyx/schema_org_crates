/// <https://schema.org/MotorcycleRepair>
pub const MOTORCYCLE_REPAIR_IRI_HTTP: &str = "http://schema.org/MotorcycleRepair";
/// <https://schema.org/MotorcycleRepair>
pub const MOTORCYCLE_REPAIR_IRI_HTTPS: &str = "https://schema.org/MotorcycleRepair";
/// <https://schema.org/MotorcycleRepair>
pub const MOTORCYCLE_REPAIR_LABEL: &str = "MotorcycleRepair";
pub struct MotorcycleRepairIri;
impl PartialEq<&str> for MotorcycleRepairIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOTORCYCLE_REPAIR_IRI_HTTP || *other == MOTORCYCLE_REPAIR_IRI_HTTPS
	}
}
impl PartialEq<MotorcycleRepairIri> for &str {
	fn eq(&self, other: &MotorcycleRepairIri) -> bool {
		*self == MOTORCYCLE_REPAIR_IRI_HTTP || *self == MOTORCYCLE_REPAIR_IRI_HTTPS
	}
}
pub struct MotorcycleRepairIriOrLabel;
impl PartialEq<&str> for MotorcycleRepairIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MotorcycleRepairIri || *other == MOTORCYCLE_REPAIR_LABEL
	}
}
impl PartialEq<MotorcycleRepairIriOrLabel> for &str {
	fn eq(&self, other: &MotorcycleRepairIriOrLabel) -> bool {
		*self == MotorcycleRepairIri || *self == MOTORCYCLE_REPAIR_LABEL
	}
}
