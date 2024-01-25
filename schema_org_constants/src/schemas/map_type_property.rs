/// <https://schema.org/mapType>
pub const MAP_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/mapType";
/// <https://schema.org/mapType>
pub const MAP_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mapType";
/// <https://schema.org/mapType>
pub const MAP_TYPE_PROPERTY_LABEL: &str = "mapType";
pub struct MapTypePropertyIri;
impl PartialEq<&str> for MapTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAP_TYPE_PROPERTY_IRI_HTTP || *other == MAP_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MapTypePropertyIri> for &str {
	fn eq(&self, other: &MapTypePropertyIri) -> bool {
		*self == MAP_TYPE_PROPERTY_IRI_HTTP || *self == MAP_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct MapTypePropertyIriOrLabel;
impl PartialEq<&str> for MapTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MapTypePropertyIri || *other == MAP_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<MapTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MapTypePropertyIriOrLabel) -> bool {
		*self == MapTypePropertyIri || *self == MAP_TYPE_PROPERTY_LABEL
	}
}
