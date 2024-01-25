/// <https://schema.org/inventoryLevel>
pub const INVENTORY_LEVEL_PROPERTY_IRI_HTTP: &str = "http://schema.org/inventoryLevel";
/// <https://schema.org/inventoryLevel>
pub const INVENTORY_LEVEL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inventoryLevel";
/// <https://schema.org/inventoryLevel>
pub const INVENTORY_LEVEL_PROPERTY_LABEL: &str = "inventoryLevel";
pub struct InventoryLevelPropertyIri;
impl PartialEq<&str> for InventoryLevelPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVENTORY_LEVEL_PROPERTY_IRI_HTTP || *other == INVENTORY_LEVEL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InventoryLevelPropertyIri> for &str {
	fn eq(&self, other: &InventoryLevelPropertyIri) -> bool {
		*self == INVENTORY_LEVEL_PROPERTY_IRI_HTTP || *self == INVENTORY_LEVEL_PROPERTY_IRI_HTTPS
	}
}
pub struct InventoryLevelPropertyIriOrLabel;
impl PartialEq<&str> for InventoryLevelPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InventoryLevelPropertyIri || *other == INVENTORY_LEVEL_PROPERTY_LABEL
	}
}
impl PartialEq<InventoryLevelPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InventoryLevelPropertyIriOrLabel) -> bool {
		*self == InventoryLevelPropertyIri || *self == INVENTORY_LEVEL_PROPERTY_LABEL
	}
}
