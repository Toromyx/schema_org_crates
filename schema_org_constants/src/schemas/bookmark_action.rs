/// <https://schema.org/BookmarkAction>
pub const BOOKMARK_ACTION_IRI_HTTP: &str = "http://schema.org/BookmarkAction";
/// <https://schema.org/BookmarkAction>
pub const BOOKMARK_ACTION_IRI_HTTPS: &str = "https://schema.org/BookmarkAction";
/// <https://schema.org/BookmarkAction>
pub const BOOKMARK_ACTION_LABEL: &str = "BookmarkAction";
pub struct BookmarkActionIri;
impl PartialEq<&str> for BookmarkActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOOKMARK_ACTION_IRI_HTTP || *other == BOOKMARK_ACTION_IRI_HTTPS
	}
}
impl PartialEq<BookmarkActionIri> for &str {
	fn eq(&self, other: &BookmarkActionIri) -> bool {
		*self == BOOKMARK_ACTION_IRI_HTTP || *self == BOOKMARK_ACTION_IRI_HTTPS
	}
}
pub struct BookmarkActionIriOrLabel;
impl PartialEq<&str> for BookmarkActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BookmarkActionIri || *other == BOOKMARK_ACTION_LABEL
	}
}
impl PartialEq<BookmarkActionIriOrLabel> for &str {
	fn eq(&self, other: &BookmarkActionIriOrLabel) -> bool {
		*self == BookmarkActionIri || *self == BOOKMARK_ACTION_LABEL
	}
}
