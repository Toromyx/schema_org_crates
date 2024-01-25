/// <https://schema.org/unitCode>
pub const UNIT_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/unitCode";
/// <https://schema.org/unitCode>
pub const UNIT_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/unitCode";
/// <https://schema.org/unitCode>
pub const UNIT_CODE_PROPERTY_LABEL: &str = "unitCode";
pub struct UnitCodePropertyIri;
impl PartialEq<&str> for UnitCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNIT_CODE_PROPERTY_IRI_HTTP || *other == UNIT_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UnitCodePropertyIri> for &str {
	fn eq(&self, other: &UnitCodePropertyIri) -> bool {
		*self == UNIT_CODE_PROPERTY_IRI_HTTP || *self == UNIT_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct UnitCodePropertyIriOrLabel;
impl PartialEq<&str> for UnitCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnitCodePropertyIri || *other == UNIT_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<UnitCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &UnitCodePropertyIriOrLabel) -> bool {
		*self == UnitCodePropertyIri || *self == UNIT_CODE_PROPERTY_LABEL
	}
}
