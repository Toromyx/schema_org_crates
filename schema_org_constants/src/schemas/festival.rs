/// <https://schema.org/Festival>
pub const FESTIVAL_IRI_HTTP: &str = "http://schema.org/Festival";
/// <https://schema.org/Festival>
pub const FESTIVAL_IRI_HTTPS: &str = "https://schema.org/Festival";
/// <https://schema.org/Festival>
pub const FESTIVAL_LABEL: &str = "Festival";
pub struct FestivalIri;
impl PartialEq<&str> for FestivalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FESTIVAL_IRI_HTTP || *other == FESTIVAL_IRI_HTTPS
	}
}
impl PartialEq<FestivalIri> for &str {
	fn eq(&self, other: &FestivalIri) -> bool {
		*self == FESTIVAL_IRI_HTTP || *self == FESTIVAL_IRI_HTTPS
	}
}
pub struct FestivalIriOrLabel;
impl PartialEq<&str> for FestivalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FestivalIri || *other == FESTIVAL_LABEL
	}
}
impl PartialEq<FestivalIriOrLabel> for &str {
	fn eq(&self, other: &FestivalIriOrLabel) -> bool {
		*self == FestivalIri || *self == FESTIVAL_LABEL
	}
}
