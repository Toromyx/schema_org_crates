/// <https://schema.org/geoEquals>
pub const GEO_EQUALS_PROPERTY_IRI_HTTP: &str = "http://schema.org/geoEquals";
/// <https://schema.org/geoEquals>
pub const GEO_EQUALS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/geoEquals";
/// <https://schema.org/geoEquals>
pub const GEO_EQUALS_PROPERTY_LABEL: &str = "geoEquals";
pub struct GeoEqualsPropertyIri;
impl PartialEq<&str> for GeoEqualsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GEO_EQUALS_PROPERTY_IRI_HTTP || *other == GEO_EQUALS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GeoEqualsPropertyIri> for &str {
	fn eq(&self, other: &GeoEqualsPropertyIri) -> bool {
		*self == GEO_EQUALS_PROPERTY_IRI_HTTP || *self == GEO_EQUALS_PROPERTY_IRI_HTTPS
	}
}
pub struct GeoEqualsPropertyIriOrLabel;
impl PartialEq<&str> for GeoEqualsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GeoEqualsPropertyIri || *other == GEO_EQUALS_PROPERTY_LABEL
	}
}
impl PartialEq<GeoEqualsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GeoEqualsPropertyIriOrLabel) -> bool {
		*self == GeoEqualsPropertyIri || *self == GEO_EQUALS_PROPERTY_LABEL
	}
}
