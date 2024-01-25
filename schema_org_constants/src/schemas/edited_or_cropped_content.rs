/// <https://schema.org/EditedOrCroppedContent>
pub const EDITED_OR_CROPPED_CONTENT_IRI_HTTP: &str = "http://schema.org/EditedOrCroppedContent";
/// <https://schema.org/EditedOrCroppedContent>
pub const EDITED_OR_CROPPED_CONTENT_IRI_HTTPS: &str = "https://schema.org/EditedOrCroppedContent";
/// <https://schema.org/EditedOrCroppedContent>
pub const EDITED_OR_CROPPED_CONTENT_LABEL: &str = "EditedOrCroppedContent";
pub struct EditedOrCroppedContentIri;
impl PartialEq<&str> for EditedOrCroppedContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EDITED_OR_CROPPED_CONTENT_IRI_HTTP
			|| *other == EDITED_OR_CROPPED_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<EditedOrCroppedContentIri> for &str {
	fn eq(&self, other: &EditedOrCroppedContentIri) -> bool {
		*self == EDITED_OR_CROPPED_CONTENT_IRI_HTTP || *self == EDITED_OR_CROPPED_CONTENT_IRI_HTTPS
	}
}
pub struct EditedOrCroppedContentIriOrLabel;
impl PartialEq<&str> for EditedOrCroppedContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EditedOrCroppedContentIri || *other == EDITED_OR_CROPPED_CONTENT_LABEL
	}
}
impl PartialEq<EditedOrCroppedContentIriOrLabel> for &str {
	fn eq(&self, other: &EditedOrCroppedContentIriOrLabel) -> bool {
		*self == EditedOrCroppedContentIri || *self == EDITED_OR_CROPPED_CONTENT_LABEL
	}
}
