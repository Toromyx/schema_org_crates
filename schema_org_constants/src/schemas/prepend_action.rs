/// <https://schema.org/PrependAction>
pub const PREPEND_ACTION_IRI_HTTP: &str = "http://schema.org/PrependAction";
/// <https://schema.org/PrependAction>
pub const PREPEND_ACTION_IRI_HTTPS: &str = "https://schema.org/PrependAction";
/// <https://schema.org/PrependAction>
pub const PREPEND_ACTION_LABEL: &str = "PrependAction";
pub struct PrependActionIri;
impl PartialEq<&str> for PrependActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREPEND_ACTION_IRI_HTTP || *other == PREPEND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<PrependActionIri> for &str {
	fn eq(&self, other: &PrependActionIri) -> bool {
		*self == PREPEND_ACTION_IRI_HTTP || *self == PREPEND_ACTION_IRI_HTTPS
	}
}
pub struct PrependActionIriOrLabel;
impl PartialEq<&str> for PrependActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrependActionIri || *other == PREPEND_ACTION_LABEL
	}
}
impl PartialEq<PrependActionIriOrLabel> for &str {
	fn eq(&self, other: &PrependActionIriOrLabel) -> bool {
		*self == PrependActionIri || *self == PREPEND_ACTION_LABEL
	}
}
