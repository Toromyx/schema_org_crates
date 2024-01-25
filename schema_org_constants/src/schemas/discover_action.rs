/// <https://schema.org/DiscoverAction>
pub const DISCOVER_ACTION_IRI_HTTP: &str = "http://schema.org/DiscoverAction";
/// <https://schema.org/DiscoverAction>
pub const DISCOVER_ACTION_IRI_HTTPS: &str = "https://schema.org/DiscoverAction";
/// <https://schema.org/DiscoverAction>
pub const DISCOVER_ACTION_LABEL: &str = "DiscoverAction";
pub struct DiscoverActionIri;
impl PartialEq<&str> for DiscoverActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISCOVER_ACTION_IRI_HTTP || *other == DISCOVER_ACTION_IRI_HTTPS
	}
}
impl PartialEq<DiscoverActionIri> for &str {
	fn eq(&self, other: &DiscoverActionIri) -> bool {
		*self == DISCOVER_ACTION_IRI_HTTP || *self == DISCOVER_ACTION_IRI_HTTPS
	}
}
pub struct DiscoverActionIriOrLabel;
impl PartialEq<&str> for DiscoverActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DiscoverActionIri || *other == DISCOVER_ACTION_LABEL
	}
}
impl PartialEq<DiscoverActionIriOrLabel> for &str {
	fn eq(&self, other: &DiscoverActionIriOrLabel) -> bool {
		*self == DiscoverActionIri || *self == DISCOVER_ACTION_LABEL
	}
}
