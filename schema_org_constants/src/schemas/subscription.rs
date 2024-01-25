/// <https://schema.org/Subscription>
pub const SUBSCRIPTION_IRI_HTTP: &str = "http://schema.org/Subscription";
/// <https://schema.org/Subscription>
pub const SUBSCRIPTION_IRI_HTTPS: &str = "https://schema.org/Subscription";
/// <https://schema.org/Subscription>
pub const SUBSCRIPTION_LABEL: &str = "Subscription";
pub struct SubscriptionIri;
impl PartialEq<&str> for SubscriptionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUBSCRIPTION_IRI_HTTP || *other == SUBSCRIPTION_IRI_HTTPS
	}
}
impl PartialEq<SubscriptionIri> for &str {
	fn eq(&self, other: &SubscriptionIri) -> bool {
		*self == SUBSCRIPTION_IRI_HTTP || *self == SUBSCRIPTION_IRI_HTTPS
	}
}
pub struct SubscriptionIriOrLabel;
impl PartialEq<&str> for SubscriptionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SubscriptionIri || *other == SUBSCRIPTION_LABEL
	}
}
impl PartialEq<SubscriptionIriOrLabel> for &str {
	fn eq(&self, other: &SubscriptionIriOrLabel) -> bool {
		*self == SubscriptionIri || *self == SUBSCRIPTION_LABEL
	}
}
