/// <https://schema.org/AppendAction>
pub const APPEND_ACTION_IRI_HTTP: &str = "http://schema.org/AppendAction";
/// <https://schema.org/AppendAction>
pub const APPEND_ACTION_IRI_HTTPS: &str = "https://schema.org/AppendAction";
/// <https://schema.org/AppendAction>
pub const APPEND_ACTION_LABEL: &str = "AppendAction";
pub struct AppendActionIri;
impl PartialEq<&str> for AppendActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPEND_ACTION_IRI_HTTP || *other == APPEND_ACTION_IRI_HTTPS
	}
}
impl PartialEq<AppendActionIri> for &str {
	fn eq(&self, other: &AppendActionIri) -> bool {
		*self == APPEND_ACTION_IRI_HTTP || *self == APPEND_ACTION_IRI_HTTPS
	}
}
pub struct AppendActionIriOrLabel;
impl PartialEq<&str> for AppendActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AppendActionIri || *other == APPEND_ACTION_LABEL
	}
}
impl PartialEq<AppendActionIriOrLabel> for &str {
	fn eq(&self, other: &AppendActionIriOrLabel) -> bool {
		*self == AppendActionIri || *self == APPEND_ACTION_LABEL
	}
}
