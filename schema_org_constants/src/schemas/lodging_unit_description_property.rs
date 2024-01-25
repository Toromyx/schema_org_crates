/// <https://schema.org/lodgingUnitDescription>
pub const LODGING_UNIT_DESCRIPTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/lodgingUnitDescription";
/// <https://schema.org/lodgingUnitDescription>
pub const LODGING_UNIT_DESCRIPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/lodgingUnitDescription";
/// <https://schema.org/lodgingUnitDescription>
pub const LODGING_UNIT_DESCRIPTION_PROPERTY_LABEL: &str = "lodgingUnitDescription";
pub struct LodgingUnitDescriptionPropertyIri;
impl PartialEq<&str> for LodgingUnitDescriptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LODGING_UNIT_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *other == LODGING_UNIT_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LodgingUnitDescriptionPropertyIri> for &str {
	fn eq(&self, other: &LodgingUnitDescriptionPropertyIri) -> bool {
		*self == LODGING_UNIT_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *self == LODGING_UNIT_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct LodgingUnitDescriptionPropertyIriOrLabel;
impl PartialEq<&str> for LodgingUnitDescriptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LodgingUnitDescriptionPropertyIri
			|| *other == LODGING_UNIT_DESCRIPTION_PROPERTY_LABEL
	}
}
impl PartialEq<LodgingUnitDescriptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LodgingUnitDescriptionPropertyIriOrLabel) -> bool {
		*self == LodgingUnitDescriptionPropertyIri
			|| *self == LODGING_UNIT_DESCRIPTION_PROPERTY_LABEL
	}
}
