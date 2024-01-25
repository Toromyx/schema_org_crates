/// <https://schema.org/unitText>
pub const UNIT_TEXT_PROPERTY_IRI_HTTP: &str = "http://schema.org/unitText";
/// <https://schema.org/unitText>
pub const UNIT_TEXT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/unitText";
/// <https://schema.org/unitText>
pub const UNIT_TEXT_PROPERTY_LABEL: &str = "unitText";
pub struct UnitTextPropertyIri;
impl PartialEq<&str> for UnitTextPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNIT_TEXT_PROPERTY_IRI_HTTP || *other == UNIT_TEXT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<UnitTextPropertyIri> for &str {
	fn eq(&self, other: &UnitTextPropertyIri) -> bool {
		*self == UNIT_TEXT_PROPERTY_IRI_HTTP || *self == UNIT_TEXT_PROPERTY_IRI_HTTPS
	}
}
pub struct UnitTextPropertyIriOrLabel;
impl PartialEq<&str> for UnitTextPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnitTextPropertyIri || *other == UNIT_TEXT_PROPERTY_LABEL
	}
}
impl PartialEq<UnitTextPropertyIriOrLabel> for &str {
	fn eq(&self, other: &UnitTextPropertyIriOrLabel) -> bool {
		*self == UnitTextPropertyIri || *self == UNIT_TEXT_PROPERTY_LABEL
	}
}
