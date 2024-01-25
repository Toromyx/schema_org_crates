/// <https://schema.org/WPAdBlock>
pub const WP_AD_BLOCK_IRI_HTTP: &str = "http://schema.org/WPAdBlock";
/// <https://schema.org/WPAdBlock>
pub const WP_AD_BLOCK_IRI_HTTPS: &str = "https://schema.org/WPAdBlock";
/// <https://schema.org/WPAdBlock>
pub const WP_AD_BLOCK_LABEL: &str = "WPAdBlock";
pub struct WpAdBlockIri;
impl PartialEq<&str> for WpAdBlockIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WP_AD_BLOCK_IRI_HTTP || *other == WP_AD_BLOCK_IRI_HTTPS
	}
}
impl PartialEq<WpAdBlockIri> for &str {
	fn eq(&self, other: &WpAdBlockIri) -> bool {
		*self == WP_AD_BLOCK_IRI_HTTP || *self == WP_AD_BLOCK_IRI_HTTPS
	}
}
pub struct WpAdBlockIriOrLabel;
impl PartialEq<&str> for WpAdBlockIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WpAdBlockIri || *other == WP_AD_BLOCK_LABEL
	}
}
impl PartialEq<WpAdBlockIriOrLabel> for &str {
	fn eq(&self, other: &WpAdBlockIriOrLabel) -> bool {
		*self == WpAdBlockIri || *self == WP_AD_BLOCK_LABEL
	}
}
