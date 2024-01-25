/// <https://schema.org/manufacturer>
pub const MANUFACTURER_PROPERTY_IRI_HTTP: &str = "http://schema.org/manufacturer";
/// <https://schema.org/manufacturer>
pub const MANUFACTURER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/manufacturer";
/// <https://schema.org/manufacturer>
pub const MANUFACTURER_PROPERTY_LABEL: &str = "manufacturer";
pub struct ManufacturerPropertyIri;
impl PartialEq<&str> for ManufacturerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MANUFACTURER_PROPERTY_IRI_HTTP || *other == MANUFACTURER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ManufacturerPropertyIri> for &str {
	fn eq(&self, other: &ManufacturerPropertyIri) -> bool {
		*self == MANUFACTURER_PROPERTY_IRI_HTTP || *self == MANUFACTURER_PROPERTY_IRI_HTTPS
	}
}
pub struct ManufacturerPropertyIriOrLabel;
impl PartialEq<&str> for ManufacturerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ManufacturerPropertyIri || *other == MANUFACTURER_PROPERTY_LABEL
	}
}
impl PartialEq<ManufacturerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ManufacturerPropertyIriOrLabel) -> bool {
		*self == ManufacturerPropertyIri || *self == MANUFACTURER_PROPERTY_LABEL
	}
}
