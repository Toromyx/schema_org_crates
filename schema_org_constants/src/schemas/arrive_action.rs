/// <https://schema.org/ArriveAction>
pub const ARRIVE_ACTION_IRI_HTTP: &str = "http://schema.org/ArriveAction";
/// <https://schema.org/ArriveAction>
pub const ARRIVE_ACTION_IRI_HTTPS: &str = "https://schema.org/ArriveAction";
/// <https://schema.org/ArriveAction>
pub const ARRIVE_ACTION_LABEL: &str = "ArriveAction";
pub struct ArriveActionIri;
impl PartialEq<&str> for ArriveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVE_ACTION_IRI_HTTP || *other == ARRIVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<ArriveActionIri> for &str {
	fn eq(&self, other: &ArriveActionIri) -> bool {
		*self == ARRIVE_ACTION_IRI_HTTP || *self == ARRIVE_ACTION_IRI_HTTPS
	}
}
pub struct ArriveActionIriOrLabel;
impl PartialEq<&str> for ArriveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArriveActionIri || *other == ARRIVE_ACTION_LABEL
	}
}
impl PartialEq<ArriveActionIriOrLabel> for &str {
	fn eq(&self, other: &ArriveActionIriOrLabel) -> bool {
		*self == ArriveActionIri || *self == ARRIVE_ACTION_LABEL
	}
}
