/// <https://schema.org/isLocatedInSubcellularLocation>
pub const IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/isLocatedInSubcellularLocation";
/// <https://schema.org/isLocatedInSubcellularLocation>
pub const IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/isLocatedInSubcellularLocation";
/// <https://schema.org/isLocatedInSubcellularLocation>
pub const IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_LABEL: &str =
	"isLocatedInSubcellularLocation";
pub struct IsLocatedInSubcellularLocationPropertyIri;
impl PartialEq<&str> for IsLocatedInSubcellularLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_IRI_HTTP
			|| *other == IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IsLocatedInSubcellularLocationPropertyIri> for &str {
	fn eq(&self, other: &IsLocatedInSubcellularLocationPropertyIri) -> bool {
		*self == IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_IRI_HTTP
			|| *self == IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct IsLocatedInSubcellularLocationPropertyIriOrLabel;
impl PartialEq<&str> for IsLocatedInSubcellularLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IsLocatedInSubcellularLocationPropertyIri
			|| *other == IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<IsLocatedInSubcellularLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &IsLocatedInSubcellularLocationPropertyIriOrLabel) -> bool {
		*self == IsLocatedInSubcellularLocationPropertyIri
			|| *self == IS_LOCATED_IN_SUBCELLULAR_LOCATION_PROPERTY_LABEL
	}
}
