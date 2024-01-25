/// <https://schema.org/PlaceOfWorship>
pub const PLACE_OF_WORSHIP_IRI_HTTP: &str = "http://schema.org/PlaceOfWorship";
/// <https://schema.org/PlaceOfWorship>
pub const PLACE_OF_WORSHIP_IRI_HTTPS: &str = "https://schema.org/PlaceOfWorship";
/// <https://schema.org/PlaceOfWorship>
pub const PLACE_OF_WORSHIP_LABEL: &str = "PlaceOfWorship";
pub struct PlaceOfWorshipIri;
impl PartialEq<&str> for PlaceOfWorshipIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLACE_OF_WORSHIP_IRI_HTTP || *other == PLACE_OF_WORSHIP_IRI_HTTPS
	}
}
impl PartialEq<PlaceOfWorshipIri> for &str {
	fn eq(&self, other: &PlaceOfWorshipIri) -> bool {
		*self == PLACE_OF_WORSHIP_IRI_HTTP || *self == PLACE_OF_WORSHIP_IRI_HTTPS
	}
}
pub struct PlaceOfWorshipIriOrLabel;
impl PartialEq<&str> for PlaceOfWorshipIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlaceOfWorshipIri || *other == PLACE_OF_WORSHIP_LABEL
	}
}
impl PartialEq<PlaceOfWorshipIriOrLabel> for &str {
	fn eq(&self, other: &PlaceOfWorshipIriOrLabel) -> bool {
		*self == PlaceOfWorshipIri || *self == PLACE_OF_WORSHIP_LABEL
	}
}
