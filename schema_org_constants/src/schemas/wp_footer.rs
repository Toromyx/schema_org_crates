/// <https://schema.org/WPFooter>
pub const WP_FOOTER_IRI_HTTP: &str = "http://schema.org/WPFooter";
/// <https://schema.org/WPFooter>
pub const WP_FOOTER_IRI_HTTPS: &str = "https://schema.org/WPFooter";
/// <https://schema.org/WPFooter>
pub const WP_FOOTER_LABEL: &str = "WPFooter";
pub struct WpFooterIri;
impl PartialEq<&str> for WpFooterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WP_FOOTER_IRI_HTTP || *other == WP_FOOTER_IRI_HTTPS
	}
}
impl PartialEq<WpFooterIri> for &str {
	fn eq(&self, other: &WpFooterIri) -> bool {
		*self == WP_FOOTER_IRI_HTTP || *self == WP_FOOTER_IRI_HTTPS
	}
}
pub struct WpFooterIriOrLabel;
impl PartialEq<&str> for WpFooterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WpFooterIri || *other == WP_FOOTER_LABEL
	}
}
impl PartialEq<WpFooterIriOrLabel> for &str {
	fn eq(&self, other: &WpFooterIriOrLabel) -> bool {
		*self == WpFooterIri || *self == WP_FOOTER_LABEL
	}
}
