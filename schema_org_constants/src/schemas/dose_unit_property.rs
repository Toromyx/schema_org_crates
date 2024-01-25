/// <https://schema.org/doseUnit>
pub const DOSE_UNIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/doseUnit";
/// <https://schema.org/doseUnit>
pub const DOSE_UNIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/doseUnit";
/// <https://schema.org/doseUnit>
pub const DOSE_UNIT_PROPERTY_LABEL: &str = "doseUnit";
pub struct DoseUnitPropertyIri;
impl PartialEq<&str> for DoseUnitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DOSE_UNIT_PROPERTY_IRI_HTTP || *other == DOSE_UNIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DoseUnitPropertyIri> for &str {
	fn eq(&self, other: &DoseUnitPropertyIri) -> bool {
		*self == DOSE_UNIT_PROPERTY_IRI_HTTP || *self == DOSE_UNIT_PROPERTY_IRI_HTTPS
	}
}
pub struct DoseUnitPropertyIriOrLabel;
impl PartialEq<&str> for DoseUnitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DoseUnitPropertyIri || *other == DOSE_UNIT_PROPERTY_LABEL
	}
}
impl PartialEq<DoseUnitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DoseUnitPropertyIriOrLabel) -> bool {
		*self == DoseUnitPropertyIri || *self == DOSE_UNIT_PROPERTY_LABEL
	}
}
