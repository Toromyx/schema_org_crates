/// <https://schema.org/SeekToAction>
pub const SEEK_TO_ACTION_IRI_HTTP: &str = "http://schema.org/SeekToAction";
/// <https://schema.org/SeekToAction>
pub const SEEK_TO_ACTION_IRI_HTTPS: &str = "https://schema.org/SeekToAction";
/// <https://schema.org/SeekToAction>
pub const SEEK_TO_ACTION_LABEL: &str = "SeekToAction";
pub struct SeekToActionIri;
impl PartialEq<&str> for SeekToActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEEK_TO_ACTION_IRI_HTTP || *other == SEEK_TO_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SeekToActionIri> for &str {
	fn eq(&self, other: &SeekToActionIri) -> bool {
		*self == SEEK_TO_ACTION_IRI_HTTP || *self == SEEK_TO_ACTION_IRI_HTTPS
	}
}
pub struct SeekToActionIriOrLabel;
impl PartialEq<&str> for SeekToActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SeekToActionIri || *other == SEEK_TO_ACTION_LABEL
	}
}
impl PartialEq<SeekToActionIriOrLabel> for &str {
	fn eq(&self, other: &SeekToActionIriOrLabel) -> bool {
		*self == SeekToActionIri || *self == SEEK_TO_ACTION_LABEL
	}
}
