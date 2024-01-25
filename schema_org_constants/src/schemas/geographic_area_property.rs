/// <https://schema.org/geographicArea>
pub const GEOGRAPHIC_AREA_PROPERTY_IRI_HTTP: &str = "http://schema.org/geographicArea";
/// <https://schema.org/geographicArea>
pub const GEOGRAPHIC_AREA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geographicArea";
/// <https://schema.org/geographicArea>
pub const GEOGRAPHIC_AREA_PROPERTY_LABEL: &str = "geographicArea";
pub struct GeographicAreaPropertyIri;
impl PartialEq<&str> for GeographicAreaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEOGRAPHIC_AREA_PROPERTY_IRI_HTTP || *other == GEOGRAPHIC_AREA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeographicAreaPropertyIri> for &str {
	fn eq(&self, other: &GeographicAreaPropertyIri) -> bool {
		*self == GEOGRAPHIC_AREA_PROPERTY_IRI_HTTP || *self == GEOGRAPHIC_AREA_PROPERTY_IRI_HTTPS
	}
}
pub struct GeographicAreaPropertyIriOrLabel;
impl PartialEq<&str> for GeographicAreaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeographicAreaPropertyIri || *other == GEOGRAPHIC_AREA_PROPERTY_LABEL
	}
}
impl PartialEq<GeographicAreaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeographicAreaPropertyIriOrLabel) -> bool {
		*self == GeographicAreaPropertyIri || *self == GEOGRAPHIC_AREA_PROPERTY_LABEL
	}
}
