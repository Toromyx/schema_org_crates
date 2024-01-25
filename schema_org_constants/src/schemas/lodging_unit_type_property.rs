/// <https://schema.org/lodgingUnitType>
pub const LODGING_UNIT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/lodgingUnitType";
/// <https://schema.org/lodgingUnitType>
pub const LODGING_UNIT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/lodgingUnitType";
/// <https://schema.org/lodgingUnitType>
pub const LODGING_UNIT_TYPE_PROPERTY_LABEL: &str = "lodgingUnitType";
pub struct LodgingUnitTypePropertyIri;
impl PartialEq<&str> for LodgingUnitTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LODGING_UNIT_TYPE_PROPERTY_IRI_HTTP
			|| *other == LODGING_UNIT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LodgingUnitTypePropertyIri> for &str {
	fn eq(&self, other: &LodgingUnitTypePropertyIri) -> bool {
		*self == LODGING_UNIT_TYPE_PROPERTY_IRI_HTTP
			|| *self == LODGING_UNIT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct LodgingUnitTypePropertyIriOrLabel;
impl PartialEq<&str> for LodgingUnitTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LodgingUnitTypePropertyIri || *other == LODGING_UNIT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<LodgingUnitTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &LodgingUnitTypePropertyIriOrLabel) -> bool {
		*self == LodgingUnitTypePropertyIri || *self == LODGING_UNIT_TYPE_PROPERTY_LABEL
	}
}
