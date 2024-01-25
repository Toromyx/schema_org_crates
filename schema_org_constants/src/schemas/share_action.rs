/// <https://schema.org/ShareAction>
pub const SHARE_ACTION_IRI_HTTP: &str = "http://schema.org/ShareAction";
/// <https://schema.org/ShareAction>
pub const SHARE_ACTION_IRI_HTTPS: &str = "https://schema.org/ShareAction";
/// <https://schema.org/ShareAction>
pub const SHARE_ACTION_LABEL: &str = "ShareAction";
pub struct ShareActionIri;
impl PartialEq<&str> for ShareActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SHARE_ACTION_IRI_HTTP || *other == SHARE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ShareActionIri> for &str {
	fn eq(&self, other: &ShareActionIri) -> bool {
		*self == SHARE_ACTION_IRI_HTTP || *self == SHARE_ACTION_IRI_HTTPS
	}
}
pub struct ShareActionIriOrLabel;
impl PartialEq<&str> for ShareActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ShareActionIri || *other == SHARE_ACTION_LABEL
	}
}
impl PartialEq<ShareActionIriOrLabel> for &str {
	fn eq(&self, other: &ShareActionIriOrLabel) -> bool {
		*self == ShareActionIri || *self == SHARE_ACTION_LABEL
	}
}
