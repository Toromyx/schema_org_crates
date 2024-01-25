/// <https://schema.org/GiveAction>
pub const GIVE_ACTION_IRI_HTTP: &str = "http://schema.org/GiveAction";
/// <https://schema.org/GiveAction>
pub const GIVE_ACTION_IRI_HTTPS: &str = "https://schema.org/GiveAction";
/// <https://schema.org/GiveAction>
pub const GIVE_ACTION_LABEL: &str = "GiveAction";
pub struct GiveActionIri;
impl PartialEq<&str> for GiveActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GIVE_ACTION_IRI_HTTP || *other == GIVE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<GiveActionIri> for &str {
	fn eq(&self, other: &GiveActionIri) -> bool {
		*self == GIVE_ACTION_IRI_HTTP || *self == GIVE_ACTION_IRI_HTTPS
	}
}
pub struct GiveActionIriOrLabel;
impl PartialEq<&str> for GiveActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GiveActionIri || *other == GIVE_ACTION_LABEL
	}
}
impl PartialEq<GiveActionIriOrLabel> for &str {
	fn eq(&self, other: &GiveActionIriOrLabel) -> bool {
		*self == GiveActionIri || *self == GIVE_ACTION_LABEL
	}
}
