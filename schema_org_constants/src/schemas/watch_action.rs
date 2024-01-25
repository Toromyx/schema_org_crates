/// <https://schema.org/WatchAction>
pub const WATCH_ACTION_IRI_HTTP: &str = "http://schema.org/WatchAction";
/// <https://schema.org/WatchAction>
pub const WATCH_ACTION_IRI_HTTPS: &str = "https://schema.org/WatchAction";
/// <https://schema.org/WatchAction>
pub const WATCH_ACTION_LABEL: &str = "WatchAction";
pub struct WatchActionIri;
impl PartialEq<&str> for WatchActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WATCH_ACTION_IRI_HTTP || *other == WATCH_ACTION_IRI_HTTPS
	}
}
impl PartialEq<WatchActionIri> for &str {
	fn eq(&self, other: &WatchActionIri) -> bool {
		*self == WATCH_ACTION_IRI_HTTP || *self == WATCH_ACTION_IRI_HTTPS
	}
}
pub struct WatchActionIriOrLabel;
impl PartialEq<&str> for WatchActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WatchActionIri || *other == WATCH_ACTION_LABEL
	}
}
impl PartialEq<WatchActionIriOrLabel> for &str {
	fn eq(&self, other: &WatchActionIriOrLabel) -> bool {
		*self == WatchActionIri || *self == WATCH_ACTION_LABEL
	}
}
