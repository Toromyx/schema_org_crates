/// <https://schema.org/OnDemandEvent>
pub const ON_DEMAND_EVENT_IRI_HTTP: &str = "http://schema.org/OnDemandEvent";
/// <https://schema.org/OnDemandEvent>
pub const ON_DEMAND_EVENT_IRI_HTTPS: &str = "https://schema.org/OnDemandEvent";
/// <https://schema.org/OnDemandEvent>
pub const ON_DEMAND_EVENT_LABEL: &str = "OnDemandEvent";
pub struct OnDemandEventIri;
impl PartialEq<&str> for OnDemandEventIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ON_DEMAND_EVENT_IRI_HTTP || *other == ON_DEMAND_EVENT_IRI_HTTPS
	}
}
impl PartialEq<OnDemandEventIri> for &str {
	fn eq(&self, other: &OnDemandEventIri) -> bool {
		*self == ON_DEMAND_EVENT_IRI_HTTP || *self == ON_DEMAND_EVENT_IRI_HTTPS
	}
}
pub struct OnDemandEventIriOrLabel;
impl PartialEq<&str> for OnDemandEventIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnDemandEventIri || *other == ON_DEMAND_EVENT_LABEL
	}
}
impl PartialEq<OnDemandEventIriOrLabel> for &str {
	fn eq(&self, other: &OnDemandEventIriOrLabel) -> bool {
		*self == OnDemandEventIri || *self == ON_DEMAND_EVENT_LABEL
	}
}
