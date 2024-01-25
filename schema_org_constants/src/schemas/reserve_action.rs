/// <https://schema.org/ReserveAction>
pub const RESERVE_ACTION_IRI_HTTP: &str = "http://schema.org/ReserveAction";
/// <https://schema.org/ReserveAction>
pub const RESERVE_ACTION_IRI_HTTPS: &str = "https://schema.org/ReserveAction";
/// <https://schema.org/ReserveAction>
pub const RESERVE_ACTION_LABEL: &str = "ReserveAction";
pub struct ReserveActionIri;
impl PartialEq<&str> for ReserveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESERVE_ACTION_IRI_HTTP || *other == RESERVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ReserveActionIri> for &str {
	fn eq(&self, other: &ReserveActionIri) -> bool {
		*self == RESERVE_ACTION_IRI_HTTP || *self == RESERVE_ACTION_IRI_HTTPS
	}
}
pub struct ReserveActionIriOrLabel;
impl PartialEq<&str> for ReserveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReserveActionIri || *other == RESERVE_ACTION_LABEL
	}
}
impl PartialEq<ReserveActionIriOrLabel> for &str {
	fn eq(&self, other: &ReserveActionIriOrLabel) -> bool {
		*self == ReserveActionIri || *self == RESERVE_ACTION_LABEL
	}
}
