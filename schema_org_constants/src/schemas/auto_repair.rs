/// <https://schema.org/AutoRepair>
pub const AUTO_REPAIR_IRI_HTTP: &str = "http://schema.org/AutoRepair";
/// <https://schema.org/AutoRepair>
pub const AUTO_REPAIR_IRI_HTTPS: &str = "https://schema.org/AutoRepair";
/// <https://schema.org/AutoRepair>
pub const AUTO_REPAIR_LABEL: &str = "AutoRepair";
pub struct AutoRepairIri;
impl PartialEq<&str> for AutoRepairIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTO_REPAIR_IRI_HTTP || *other == AUTO_REPAIR_IRI_HTTPS
	}
}
impl PartialEq<AutoRepairIri> for &str {
	fn eq(&self, other: &AutoRepairIri) -> bool {
		*self == AUTO_REPAIR_IRI_HTTP || *self == AUTO_REPAIR_IRI_HTTPS
	}
}
pub struct AutoRepairIriOrLabel;
impl PartialEq<&str> for AutoRepairIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AutoRepairIri || *other == AUTO_REPAIR_LABEL
	}
}
impl PartialEq<AutoRepairIriOrLabel> for &str {
	fn eq(&self, other: &AutoRepairIriOrLabel) -> bool {
		*self == AutoRepairIri || *self == AUTO_REPAIR_LABEL
	}
}
