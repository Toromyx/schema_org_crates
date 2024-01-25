/// <https://schema.org/medicineSystem>
pub const MEDICINE_SYSTEM_PROPERTY_IRI_HTTP: &str = "http://schema.org/medicineSystem";
/// <https://schema.org/medicineSystem>
pub const MEDICINE_SYSTEM_PROPERTY_IRI_HTTPS: &str = "https://schema.org/medicineSystem";
/// <https://schema.org/medicineSystem>
pub const MEDICINE_SYSTEM_PROPERTY_LABEL: &str = "medicineSystem";
pub struct MedicineSystemPropertyIri;
impl PartialEq<&str> for MedicineSystemPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICINE_SYSTEM_PROPERTY_IRI_HTTP || *other == MEDICINE_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MedicineSystemPropertyIri> for &str {
	fn eq(&self, other: &MedicineSystemPropertyIri) -> bool {
		*self == MEDICINE_SYSTEM_PROPERTY_IRI_HTTP || *self == MEDICINE_SYSTEM_PROPERTY_IRI_HTTPS
	}
}
pub struct MedicineSystemPropertyIriOrLabel;
impl PartialEq<&str> for MedicineSystemPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicineSystemPropertyIri || *other == MEDICINE_SYSTEM_PROPERTY_LABEL
	}
}
impl PartialEq<MedicineSystemPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MedicineSystemPropertyIriOrLabel) -> bool {
		*self == MedicineSystemPropertyIri || *self == MEDICINE_SYSTEM_PROPERTY_LABEL
	}
}
