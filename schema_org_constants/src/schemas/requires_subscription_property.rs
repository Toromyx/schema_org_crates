/// <https://schema.org/requiresSubscription>
pub const REQUIRES_SUBSCRIPTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/requiresSubscription";
/// <https://schema.org/requiresSubscription>
pub const REQUIRES_SUBSCRIPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/requiresSubscription";
/// <https://schema.org/requiresSubscription>
pub const REQUIRES_SUBSCRIPTION_PROPERTY_LABEL: &str = "requiresSubscription";
pub struct RequiresSubscriptionPropertyIri;
impl PartialEq<&str> for RequiresSubscriptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REQUIRES_SUBSCRIPTION_PROPERTY_IRI_HTTP
			|| *other == REQUIRES_SUBSCRIPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RequiresSubscriptionPropertyIri> for &str {
	fn eq(&self, other: &RequiresSubscriptionPropertyIri) -> bool {
		*self == REQUIRES_SUBSCRIPTION_PROPERTY_IRI_HTTP
			|| *self == REQUIRES_SUBSCRIPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct RequiresSubscriptionPropertyIriOrLabel;
impl PartialEq<&str> for RequiresSubscriptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RequiresSubscriptionPropertyIri || *other == REQUIRES_SUBSCRIPTION_PROPERTY_LABEL
	}
}
impl PartialEq<RequiresSubscriptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RequiresSubscriptionPropertyIriOrLabel) -> bool {
		*self == RequiresSubscriptionPropertyIri || *self == REQUIRES_SUBSCRIPTION_PROPERTY_LABEL
	}
}
