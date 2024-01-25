/// <https://schema.org/MedicineSystem>
pub const MEDICINE_SYSTEM_IRI_HTTP: &str = "http://schema.org/MedicineSystem";
/// <https://schema.org/MedicineSystem>
pub const MEDICINE_SYSTEM_IRI_HTTPS: &str = "https://schema.org/MedicineSystem";
/// <https://schema.org/MedicineSystem>
pub const MEDICINE_SYSTEM_LABEL: &str = "MedicineSystem";
pub struct MedicineSystemIri;
impl PartialEq<&str> for MedicineSystemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDICINE_SYSTEM_IRI_HTTP || *other == MEDICINE_SYSTEM_IRI_HTTPS
	}
}
impl PartialEq<MedicineSystemIri> for &str {
	fn eq(&self, other: &MedicineSystemIri) -> bool {
		*self == MEDICINE_SYSTEM_IRI_HTTP || *self == MEDICINE_SYSTEM_IRI_HTTPS
	}
}
pub struct MedicineSystemIriOrLabel;
impl PartialEq<&str> for MedicineSystemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedicineSystemIri || *other == MEDICINE_SYSTEM_LABEL
	}
}
impl PartialEq<MedicineSystemIriOrLabel> for &str {
	fn eq(&self, other: &MedicineSystemIriOrLabel) -> bool {
		*self == MedicineSystemIri || *self == MEDICINE_SYSTEM_LABEL
	}
}
