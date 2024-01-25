/// <https://schema.org/strengthUnit>
pub const STRENGTH_UNIT_PROPERTY_IRI_HTTP: &str = "http://schema.org/strengthUnit";
/// <https://schema.org/strengthUnit>
pub const STRENGTH_UNIT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/strengthUnit";
/// <https://schema.org/strengthUnit>
pub const STRENGTH_UNIT_PROPERTY_LABEL: &str = "strengthUnit";
pub struct StrengthUnitPropertyIri;
impl PartialEq<&str> for StrengthUnitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STRENGTH_UNIT_PROPERTY_IRI_HTTP || *other == STRENGTH_UNIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StrengthUnitPropertyIri> for &str {
	fn eq(&self, other: &StrengthUnitPropertyIri) -> bool {
		*self == STRENGTH_UNIT_PROPERTY_IRI_HTTP || *self == STRENGTH_UNIT_PROPERTY_IRI_HTTPS
	}
}
pub struct StrengthUnitPropertyIriOrLabel;
impl PartialEq<&str> for StrengthUnitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StrengthUnitPropertyIri || *other == STRENGTH_UNIT_PROPERTY_LABEL
	}
}
impl PartialEq<StrengthUnitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StrengthUnitPropertyIriOrLabel) -> bool {
		*self == StrengthUnitPropertyIri || *self == STRENGTH_UNIT_PROPERTY_LABEL
	}
}
