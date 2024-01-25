/// <https://schema.org/Place>
pub const PLACE_IRI_HTTP: &str = "http://schema.org/Place";
/// <https://schema.org/Place>
pub const PLACE_IRI_HTTPS: &str = "https://schema.org/Place";
/// <https://schema.org/Place>
pub const PLACE_LABEL: &str = "Place";
pub struct PlaceIri;
impl PartialEq<&str> for PlaceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLACE_IRI_HTTP || *other == PLACE_IRI_HTTPS
	}
}
impl PartialEq<PlaceIri> for &str {
	fn eq(&self, other: &PlaceIri) -> bool {
		*self == PLACE_IRI_HTTP || *self == PLACE_IRI_HTTPS
	}
}
pub struct PlaceIriOrLabel;
impl PartialEq<&str> for PlaceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlaceIri || *other == PLACE_LABEL
	}
}
impl PartialEq<PlaceIriOrLabel> for &str {
	fn eq(&self, other: &PlaceIriOrLabel) -> bool {
		*self == PlaceIri || *self == PLACE_LABEL
	}
}
