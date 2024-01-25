/// <https://schema.org/Winery>
pub const WINERY_IRI_HTTP: &str = "http://schema.org/Winery";
/// <https://schema.org/Winery>
pub const WINERY_IRI_HTTPS: &str = "https://schema.org/Winery";
/// <https://schema.org/Winery>
pub const WINERY_LABEL: &str = "Winery";
pub struct WineryIri;
impl PartialEq<&str> for WineryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WINERY_IRI_HTTP || *other == WINERY_IRI_HTTPS
	}
}
impl PartialEq<WineryIri> for &str {
	fn eq(&self, other: &WineryIri) -> bool {
		*self == WINERY_IRI_HTTP || *self == WINERY_IRI_HTTPS
	}
}
pub struct WineryIriOrLabel;
impl PartialEq<&str> for WineryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WineryIri || *other == WINERY_LABEL
	}
}
impl PartialEq<WineryIriOrLabel> for &str {
	fn eq(&self, other: &WineryIriOrLabel) -> bool {
		*self == WineryIri || *self == WINERY_LABEL
	}
}
