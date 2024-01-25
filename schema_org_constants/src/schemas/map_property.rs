/// <https://schema.org/map>
#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
pub const MAP_PROPERTY_IRI_HTTP: &str = "http://schema.org/map";
/// <https://schema.org/map>
#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
pub const MAP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/map";
/// <https://schema.org/map>
#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
pub const MAP_PROPERTY_LABEL: &str = "map";
pub struct MapPropertyIri;
impl PartialEq<&str> for MapPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAP_PROPERTY_IRI_HTTP || *other == MAP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MapPropertyIri> for &str {
	fn eq(&self, other: &MapPropertyIri) -> bool {
		*self == MAP_PROPERTY_IRI_HTTP || *self == MAP_PROPERTY_IRI_HTTPS
	}
}
pub struct MapPropertyIriOrLabel;
impl PartialEq<&str> for MapPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MapPropertyIri || *other == MAP_PROPERTY_LABEL
	}
}
impl PartialEq<MapPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MapPropertyIriOrLabel) -> bool {
		*self == MapPropertyIri || *self == MAP_PROPERTY_LABEL
	}
}
