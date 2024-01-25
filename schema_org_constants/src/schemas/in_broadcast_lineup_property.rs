/// <https://schema.org/inBroadcastLineup>
pub const IN_BROADCAST_LINEUP_PROPERTY_IRI_HTTP: &str = "http://schema.org/inBroadcastLineup";
/// <https://schema.org/inBroadcastLineup>
pub const IN_BROADCAST_LINEUP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inBroadcastLineup";
/// <https://schema.org/inBroadcastLineup>
pub const IN_BROADCAST_LINEUP_PROPERTY_LABEL: &str = "inBroadcastLineup";
pub struct InBroadcastLineupPropertyIri;
impl PartialEq<&str> for InBroadcastLineupPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_BROADCAST_LINEUP_PROPERTY_IRI_HTTP
			|| *other == IN_BROADCAST_LINEUP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InBroadcastLineupPropertyIri> for &str {
	fn eq(&self, other: &InBroadcastLineupPropertyIri) -> bool {
		*self == IN_BROADCAST_LINEUP_PROPERTY_IRI_HTTP
			|| *self == IN_BROADCAST_LINEUP_PROPERTY_IRI_HTTPS
	}
}
pub struct InBroadcastLineupPropertyIriOrLabel;
impl PartialEq<&str> for InBroadcastLineupPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InBroadcastLineupPropertyIri || *other == IN_BROADCAST_LINEUP_PROPERTY_LABEL
	}
}
impl PartialEq<InBroadcastLineupPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InBroadcastLineupPropertyIriOrLabel) -> bool {
		*self == InBroadcastLineupPropertyIri || *self == IN_BROADCAST_LINEUP_PROPERTY_LABEL
	}
}
