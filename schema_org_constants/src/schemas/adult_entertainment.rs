/// <https://schema.org/AdultEntertainment>
pub const ADULT_ENTERTAINMENT_IRI_HTTP: &str = "http://schema.org/AdultEntertainment";
/// <https://schema.org/AdultEntertainment>
pub const ADULT_ENTERTAINMENT_IRI_HTTPS: &str = "https://schema.org/AdultEntertainment";
/// <https://schema.org/AdultEntertainment>
pub const ADULT_ENTERTAINMENT_LABEL: &str = "AdultEntertainment";
pub struct AdultEntertainmentIri;
impl PartialEq<&str> for AdultEntertainmentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADULT_ENTERTAINMENT_IRI_HTTP || *other == ADULT_ENTERTAINMENT_IRI_HTTPS
	}
}
impl PartialEq<AdultEntertainmentIri> for &str {
	fn eq(&self, other: &AdultEntertainmentIri) -> bool {
		*self == ADULT_ENTERTAINMENT_IRI_HTTP || *self == ADULT_ENTERTAINMENT_IRI_HTTPS
	}
}
pub struct AdultEntertainmentIriOrLabel;
impl PartialEq<&str> for AdultEntertainmentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdultEntertainmentIri || *other == ADULT_ENTERTAINMENT_LABEL
	}
}
impl PartialEq<AdultEntertainmentIriOrLabel> for &str {
	fn eq(&self, other: &AdultEntertainmentIriOrLabel) -> bool {
		*self == AdultEntertainmentIri || *self == ADULT_ENTERTAINMENT_LABEL
	}
}
