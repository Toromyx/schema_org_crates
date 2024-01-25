/// <https://schema.org/Campground>
pub const CAMPGROUND_IRI_HTTP: &str = "http://schema.org/Campground";
/// <https://schema.org/Campground>
pub const CAMPGROUND_IRI_HTTPS: &str = "https://schema.org/Campground";
/// <https://schema.org/Campground>
pub const CAMPGROUND_LABEL: &str = "Campground";
pub struct CampgroundIri;
impl PartialEq<&str> for CampgroundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAMPGROUND_IRI_HTTP || *other == CAMPGROUND_IRI_HTTPS
	}
}
impl PartialEq<CampgroundIri> for &str {
	fn eq(&self, other: &CampgroundIri) -> bool {
		*self == CAMPGROUND_IRI_HTTP || *self == CAMPGROUND_IRI_HTTPS
	}
}
pub struct CampgroundIriOrLabel;
impl PartialEq<&str> for CampgroundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CampgroundIri || *other == CAMPGROUND_LABEL
	}
}
impl PartialEq<CampgroundIriOrLabel> for &str {
	fn eq(&self, other: &CampgroundIriOrLabel) -> bool {
		*self == CampgroundIri || *self == CAMPGROUND_LABEL
	}
}
