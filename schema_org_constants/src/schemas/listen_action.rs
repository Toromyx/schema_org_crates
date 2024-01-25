/// <https://schema.org/ListenAction>
pub const LISTEN_ACTION_IRI_HTTP: &str = "http://schema.org/ListenAction";
/// <https://schema.org/ListenAction>
pub const LISTEN_ACTION_IRI_HTTPS: &str = "https://schema.org/ListenAction";
/// <https://schema.org/ListenAction>
pub const LISTEN_ACTION_LABEL: &str = "ListenAction";
pub struct ListenActionIri;
impl PartialEq<&str> for ListenActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LISTEN_ACTION_IRI_HTTP || *other == LISTEN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ListenActionIri> for &str {
	fn eq(&self, other: &ListenActionIri) -> bool {
		*self == LISTEN_ACTION_IRI_HTTP || *self == LISTEN_ACTION_IRI_HTTPS
	}
}
pub struct ListenActionIriOrLabel;
impl PartialEq<&str> for ListenActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ListenActionIri || *other == LISTEN_ACTION_LABEL
	}
}
impl PartialEq<ListenActionIriOrLabel> for &str {
	fn eq(&self, other: &ListenActionIriOrLabel) -> bool {
		*self == ListenActionIri || *self == LISTEN_ACTION_LABEL
	}
}
