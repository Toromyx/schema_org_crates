/// <https://schema.org/polygon>
pub const POLYGON_PROPERTY_IRI_HTTP: &str = "http://schema.org/polygon";
/// <https://schema.org/polygon>
pub const POLYGON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/polygon";
/// <https://schema.org/polygon>
pub const POLYGON_PROPERTY_LABEL: &str = "polygon";
pub struct PolygonPropertyIri;
impl PartialEq<&str> for PolygonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POLYGON_PROPERTY_IRI_HTTP || *other == POLYGON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PolygonPropertyIri> for &str {
	fn eq(&self, other: &PolygonPropertyIri) -> bool {
		*self == POLYGON_PROPERTY_IRI_HTTP || *self == POLYGON_PROPERTY_IRI_HTTPS
	}
}
pub struct PolygonPropertyIriOrLabel;
impl PartialEq<&str> for PolygonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PolygonPropertyIri || *other == POLYGON_PROPERTY_LABEL
	}
}
impl PartialEq<PolygonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PolygonPropertyIriOrLabel) -> bool {
		*self == PolygonPropertyIri || *self == POLYGON_PROPERTY_LABEL
	}
}
