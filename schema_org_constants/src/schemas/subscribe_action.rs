/// <https://schema.org/SubscribeAction>
pub const SUBSCRIBE_ACTION_IRI_HTTP: &str = "http://schema.org/SubscribeAction";
/// <https://schema.org/SubscribeAction>
pub const SUBSCRIBE_ACTION_IRI_HTTPS: &str = "https://schema.org/SubscribeAction";
/// <https://schema.org/SubscribeAction>
pub const SUBSCRIBE_ACTION_LABEL: &str = "SubscribeAction";
pub struct SubscribeActionIri;
impl PartialEq<&str> for SubscribeActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUBSCRIBE_ACTION_IRI_HTTP || *other == SUBSCRIBE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SubscribeActionIri> for &str {
	fn eq(&self, other: &SubscribeActionIri) -> bool {
		*self == SUBSCRIBE_ACTION_IRI_HTTP || *self == SUBSCRIBE_ACTION_IRI_HTTPS
	}
}
pub struct SubscribeActionIriOrLabel;
impl PartialEq<&str> for SubscribeActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubscribeActionIri || *other == SUBSCRIBE_ACTION_LABEL
	}
}
impl PartialEq<SubscribeActionIriOrLabel> for &str {
	fn eq(&self, other: &SubscribeActionIriOrLabel) -> bool {
		*self == SubscribeActionIri || *self == SUBSCRIBE_ACTION_LABEL
	}
}
