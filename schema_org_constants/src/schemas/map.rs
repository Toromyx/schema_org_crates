/// <https://schema.org/Map>
pub const MAP_IRI_HTTP: &str = "http://schema.org/Map";
/// <https://schema.org/Map>
pub const MAP_IRI_HTTPS: &str = "https://schema.org/Map";
/// <https://schema.org/Map>
pub const MAP_LABEL: &str = "Map";
pub struct MapIri;
impl PartialEq<&str> for MapIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAP_IRI_HTTP || *other == MAP_IRI_HTTPS
	}
}
impl PartialEq<MapIri> for &str {
	fn eq(&self, other: &MapIri) -> bool {
		*self == MAP_IRI_HTTP || *self == MAP_IRI_HTTPS
	}
}
pub struct MapIriOrLabel;
impl PartialEq<&str> for MapIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MapIri || *other == MAP_LABEL
	}
}
impl PartialEq<MapIriOrLabel> for &str {
	fn eq(&self, other: &MapIriOrLabel) -> bool {
		*self == MapIri || *self == MAP_LABEL
	}
}
