/// <https://schema.org/InviteAction>
pub const INVITE_ACTION_IRI_HTTP: &str = "http://schema.org/InviteAction";
/// <https://schema.org/InviteAction>
pub const INVITE_ACTION_IRI_HTTPS: &str = "https://schema.org/InviteAction";
/// <https://schema.org/InviteAction>
pub const INVITE_ACTION_LABEL: &str = "InviteAction";
pub struct InviteActionIri;
impl PartialEq<&str> for InviteActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INVITE_ACTION_IRI_HTTP || *other == INVITE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<InviteActionIri> for &str {
	fn eq(&self, other: &InviteActionIri) -> bool {
		*self == INVITE_ACTION_IRI_HTTP || *self == INVITE_ACTION_IRI_HTTPS
	}
}
pub struct InviteActionIriOrLabel;
impl PartialEq<&str> for InviteActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InviteActionIri || *other == INVITE_ACTION_LABEL
	}
}
impl PartialEq<InviteActionIriOrLabel> for &str {
	fn eq(&self, other: &InviteActionIriOrLabel) -> bool {
		*self == InviteActionIri || *self == INVITE_ACTION_LABEL
	}
}
