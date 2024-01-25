/// <https://schema.org/CassetteFormat>
pub const CASSETTE_FORMAT_IRI_HTTP: &str = "http://schema.org/CassetteFormat";
/// <https://schema.org/CassetteFormat>
pub const CASSETTE_FORMAT_IRI_HTTPS: &str = "https://schema.org/CassetteFormat";
/// <https://schema.org/CassetteFormat>
pub const CASSETTE_FORMAT_LABEL: &str = "CassetteFormat";
pub struct CassetteFormatIri;
impl PartialEq<&str> for CassetteFormatIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CASSETTE_FORMAT_IRI_HTTP || *other == CASSETTE_FORMAT_IRI_HTTPS
	}
}
impl PartialEq<CassetteFormatIri> for &str {
	fn eq(&self, other: &CassetteFormatIri) -> bool {
		*self == CASSETTE_FORMAT_IRI_HTTP || *self == CASSETTE_FORMAT_IRI_HTTPS
	}
}
pub struct CassetteFormatIriOrLabel;
impl PartialEq<&str> for CassetteFormatIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CassetteFormatIri || *other == CASSETTE_FORMAT_LABEL
	}
}
impl PartialEq<CassetteFormatIriOrLabel> for &str {
	fn eq(&self, other: &CassetteFormatIriOrLabel) -> bool {
		*self == CassetteFormatIri || *self == CASSETTE_FORMAT_LABEL
	}
}
