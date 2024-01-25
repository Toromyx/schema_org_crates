/// <https://schema.org/TouristAttraction>
pub const TOURIST_ATTRACTION_IRI_HTTP: &str = "http://schema.org/TouristAttraction";
/// <https://schema.org/TouristAttraction>
pub const TOURIST_ATTRACTION_IRI_HTTPS: &str = "https://schema.org/TouristAttraction";
/// <https://schema.org/TouristAttraction>
pub const TOURIST_ATTRACTION_LABEL: &str = "TouristAttraction";
pub struct TouristAttractionIri;
impl PartialEq<&str> for TouristAttractionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOURIST_ATTRACTION_IRI_HTTP || *other == TOURIST_ATTRACTION_IRI_HTTPS
	}
}
impl PartialEq<TouristAttractionIri> for &str {
	fn eq(&self, other: &TouristAttractionIri) -> bool {
		*self == TOURIST_ATTRACTION_IRI_HTTP || *self == TOURIST_ATTRACTION_IRI_HTTPS
	}
}
pub struct TouristAttractionIriOrLabel;
impl PartialEq<&str> for TouristAttractionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TouristAttractionIri || *other == TOURIST_ATTRACTION_LABEL
	}
}
impl PartialEq<TouristAttractionIriOrLabel> for &str {
	fn eq(&self, other: &TouristAttractionIriOrLabel) -> bool {
		*self == TouristAttractionIri || *self == TOURIST_ATTRACTION_LABEL
	}
}
