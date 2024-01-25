/// <https://schema.org/WPSideBar>
pub const WP_SIDE_BAR_IRI_HTTP: &str = "http://schema.org/WPSideBar";
/// <https://schema.org/WPSideBar>
pub const WP_SIDE_BAR_IRI_HTTPS: &str = "https://schema.org/WPSideBar";
/// <https://schema.org/WPSideBar>
pub const WP_SIDE_BAR_LABEL: &str = "WPSideBar";
pub struct WpSideBarIri;
impl PartialEq<&str> for WpSideBarIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WP_SIDE_BAR_IRI_HTTP || *other == WP_SIDE_BAR_IRI_HTTPS
	}
}
impl PartialEq<WpSideBarIri> for &str {
	fn eq(&self, other: &WpSideBarIri) -> bool {
		*self == WP_SIDE_BAR_IRI_HTTP || *self == WP_SIDE_BAR_IRI_HTTPS
	}
}
pub struct WpSideBarIriOrLabel;
impl PartialEq<&str> for WpSideBarIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WpSideBarIri || *other == WP_SIDE_BAR_LABEL
	}
}
impl PartialEq<WpSideBarIriOrLabel> for &str {
	fn eq(&self, other: &WpSideBarIriOrLabel) -> bool {
		*self == WpSideBarIri || *self == WP_SIDE_BAR_LABEL
	}
}
