/// <https://schema.org/OfficeEquipmentStore>
pub const OFFICE_EQUIPMENT_STORE_IRI_HTTP: &str = "http://schema.org/OfficeEquipmentStore";
/// <https://schema.org/OfficeEquipmentStore>
pub const OFFICE_EQUIPMENT_STORE_IRI_HTTPS: &str = "https://schema.org/OfficeEquipmentStore";
/// <https://schema.org/OfficeEquipmentStore>
pub const OFFICE_EQUIPMENT_STORE_LABEL: &str = "OfficeEquipmentStore";
pub struct OfficeEquipmentStoreIri;
impl PartialEq<&str> for OfficeEquipmentStoreIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFICE_EQUIPMENT_STORE_IRI_HTTP || *other == OFFICE_EQUIPMENT_STORE_IRI_HTTPS
	}
}
impl PartialEq<OfficeEquipmentStoreIri> for &str {
	fn eq(&self, other: &OfficeEquipmentStoreIri) -> bool {
		*self == OFFICE_EQUIPMENT_STORE_IRI_HTTP || *self == OFFICE_EQUIPMENT_STORE_IRI_HTTPS
	}
}
pub struct OfficeEquipmentStoreIriOrLabel;
impl PartialEq<&str> for OfficeEquipmentStoreIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfficeEquipmentStoreIri || *other == OFFICE_EQUIPMENT_STORE_LABEL
	}
}
impl PartialEq<OfficeEquipmentStoreIriOrLabel> for &str {
	fn eq(&self, other: &OfficeEquipmentStoreIriOrLabel) -> bool {
		*self == OfficeEquipmentStoreIri || *self == OFFICE_EQUIPMENT_STORE_LABEL
	}
}
