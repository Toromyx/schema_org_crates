/// <https://schema.org/providesBroadcastService>
pub const PROVIDES_BROADCAST_SERVICE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/providesBroadcastService";
/// <https://schema.org/providesBroadcastService>
pub const PROVIDES_BROADCAST_SERVICE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/providesBroadcastService";
/// <https://schema.org/providesBroadcastService>
pub const PROVIDES_BROADCAST_SERVICE_PROPERTY_LABEL: &str = "providesBroadcastService";
pub struct ProvidesBroadcastServicePropertyIri;
impl PartialEq<&str> for ProvidesBroadcastServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROVIDES_BROADCAST_SERVICE_PROPERTY_IRI_HTTP
			|| *other == PROVIDES_BROADCAST_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProvidesBroadcastServicePropertyIri> for &str {
	fn eq(&self, other: &ProvidesBroadcastServicePropertyIri) -> bool {
		*self == PROVIDES_BROADCAST_SERVICE_PROPERTY_IRI_HTTP
			|| *self == PROVIDES_BROADCAST_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProvidesBroadcastServicePropertyIriOrLabel;
impl PartialEq<&str> for ProvidesBroadcastServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProvidesBroadcastServicePropertyIri
			|| *other == PROVIDES_BROADCAST_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<ProvidesBroadcastServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProvidesBroadcastServicePropertyIriOrLabel) -> bool {
		*self == ProvidesBroadcastServicePropertyIri
			|| *self == PROVIDES_BROADCAST_SERVICE_PROPERTY_LABEL
	}
}
