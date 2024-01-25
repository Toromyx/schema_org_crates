/// <https://schema.org/WPHeader>
pub const WP_HEADER_IRI_HTTP: &str = "http://schema.org/WPHeader";
/// <https://schema.org/WPHeader>
pub const WP_HEADER_IRI_HTTPS: &str = "https://schema.org/WPHeader";
/// <https://schema.org/WPHeader>
pub const WP_HEADER_LABEL: &str = "WPHeader";
pub struct WpHeaderIri;
impl PartialEq<&str> for WpHeaderIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WP_HEADER_IRI_HTTP || *other == WP_HEADER_IRI_HTTPS
	}
}
impl PartialEq<WpHeaderIri> for &str {
	fn eq(&self, other: &WpHeaderIri) -> bool {
		*self == WP_HEADER_IRI_HTTP || *self == WP_HEADER_IRI_HTTPS
	}
}
pub struct WpHeaderIriOrLabel;
impl PartialEq<&str> for WpHeaderIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WpHeaderIri || *other == WP_HEADER_LABEL
	}
}
impl PartialEq<WpHeaderIriOrLabel> for &str {
	fn eq(&self, other: &WpHeaderIriOrLabel) -> bool {
		*self == WpHeaderIri || *self == WP_HEADER_LABEL
	}
}
