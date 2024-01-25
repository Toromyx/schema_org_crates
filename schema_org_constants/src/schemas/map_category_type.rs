/// <https://schema.org/MapCategoryType>
pub const MAP_CATEGORY_TYPE_IRI_HTTP: &str = "http://schema.org/MapCategoryType";
/// <https://schema.org/MapCategoryType>
pub const MAP_CATEGORY_TYPE_IRI_HTTPS: &str = "https://schema.org/MapCategoryType";
/// <https://schema.org/MapCategoryType>
pub const MAP_CATEGORY_TYPE_LABEL: &str = "MapCategoryType";
pub struct MapCategoryTypeIri;
impl PartialEq<&str> for MapCategoryTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAP_CATEGORY_TYPE_IRI_HTTP || *other == MAP_CATEGORY_TYPE_IRI_HTTPS
	}
}
impl PartialEq<MapCategoryTypeIri> for &str {
	fn eq(&self, other: &MapCategoryTypeIri) -> bool {
		*self == MAP_CATEGORY_TYPE_IRI_HTTP || *self == MAP_CATEGORY_TYPE_IRI_HTTPS
	}
}
pub struct MapCategoryTypeIriOrLabel;
impl PartialEq<&str> for MapCategoryTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MapCategoryTypeIri || *other == MAP_CATEGORY_TYPE_LABEL
	}
}
impl PartialEq<MapCategoryTypeIriOrLabel> for &str {
	fn eq(&self, other: &MapCategoryTypeIriOrLabel) -> bool {
		*self == MapCategoryTypeIri || *self == MAP_CATEGORY_TYPE_LABEL
	}
}
