/// <https://schema.org/InternetCafe>
pub const INTERNET_CAFE_IRI_HTTP: &str = "http://schema.org/InternetCafe";
/// <https://schema.org/InternetCafe>
pub const INTERNET_CAFE_IRI_HTTPS: &str = "https://schema.org/InternetCafe";
/// <https://schema.org/InternetCafe>
pub const INTERNET_CAFE_LABEL: &str = "InternetCafe";
pub struct InternetCafeIri;
impl PartialEq<&str> for InternetCafeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERNET_CAFE_IRI_HTTP || *other == INTERNET_CAFE_IRI_HTTPS
	}
}
impl PartialEq<InternetCafeIri> for &str {
	fn eq(&self, other: &InternetCafeIri) -> bool {
		*self == INTERNET_CAFE_IRI_HTTP || *self == INTERNET_CAFE_IRI_HTTPS
	}
}
pub struct InternetCafeIriOrLabel;
impl PartialEq<&str> for InternetCafeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InternetCafeIri || *other == INTERNET_CAFE_LABEL
	}
}
impl PartialEq<InternetCafeIriOrLabel> for &str {
	fn eq(&self, other: &InternetCafeIriOrLabel) -> bool {
		*self == InternetCafeIri || *self == INTERNET_CAFE_LABEL
	}
}
