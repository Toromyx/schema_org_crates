/// <https://schema.org/OnSitePickup>
pub const ON_SITE_PICKUP_IRI_HTTP: &str = "http://schema.org/OnSitePickup";
/// <https://schema.org/OnSitePickup>
pub const ON_SITE_PICKUP_IRI_HTTPS: &str = "https://schema.org/OnSitePickup";
/// <https://schema.org/OnSitePickup>
pub const ON_SITE_PICKUP_LABEL: &str = "OnSitePickup";
pub struct OnSitePickupIri;
impl PartialEq<&str> for OnSitePickupIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ON_SITE_PICKUP_IRI_HTTP || *other == ON_SITE_PICKUP_IRI_HTTPS
	}
}
impl PartialEq<OnSitePickupIri> for &str {
	fn eq(&self, other: &OnSitePickupIri) -> bool {
		*self == ON_SITE_PICKUP_IRI_HTTP || *self == ON_SITE_PICKUP_IRI_HTTPS
	}
}
pub struct OnSitePickupIriOrLabel;
impl PartialEq<&str> for OnSitePickupIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnSitePickupIri || *other == ON_SITE_PICKUP_LABEL
	}
}
impl PartialEq<OnSitePickupIriOrLabel> for &str {
	fn eq(&self, other: &OnSitePickupIriOrLabel) -> bool {
		*self == OnSitePickupIri || *self == ON_SITE_PICKUP_LABEL
	}
}
