/// <https://schema.org/WearAction>
pub const WEAR_ACTION_IRI_HTTP: &str = "http://schema.org/WearAction";
/// <https://schema.org/WearAction>
pub const WEAR_ACTION_IRI_HTTPS: &str = "https://schema.org/WearAction";
/// <https://schema.org/WearAction>
pub const WEAR_ACTION_LABEL: &str = "WearAction";
pub struct WearActionIri;
impl PartialEq<&str> for WearActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WEAR_ACTION_IRI_HTTP || *other == WEAR_ACTION_IRI_HTTPS
	}
}
impl PartialEq<WearActionIri> for &str {
	fn eq(&self, other: &WearActionIri) -> bool {
		*self == WEAR_ACTION_IRI_HTTP || *self == WEAR_ACTION_IRI_HTTPS
	}
}
pub struct WearActionIriOrLabel;
impl PartialEq<&str> for WearActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WearActionIri || *other == WEAR_ACTION_LABEL
	}
}
impl PartialEq<WearActionIriOrLabel> for &str {
	fn eq(&self, other: &WearActionIriOrLabel) -> bool {
		*self == WearActionIri || *self == WEAR_ACTION_LABEL
	}
}
