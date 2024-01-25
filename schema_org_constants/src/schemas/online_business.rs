/// <https://schema.org/OnlineBusiness>
pub const ONLINE_BUSINESS_IRI_HTTP: &str = "http://schema.org/OnlineBusiness";
/// <https://schema.org/OnlineBusiness>
pub const ONLINE_BUSINESS_IRI_HTTPS: &str = "https://schema.org/OnlineBusiness";
/// <https://schema.org/OnlineBusiness>
pub const ONLINE_BUSINESS_LABEL: &str = "OnlineBusiness";
pub struct OnlineBusinessIri;
impl PartialEq<&str> for OnlineBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONLINE_BUSINESS_IRI_HTTP || *other == ONLINE_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<OnlineBusinessIri> for &str {
	fn eq(&self, other: &OnlineBusinessIri) -> bool {
		*self == ONLINE_BUSINESS_IRI_HTTP || *self == ONLINE_BUSINESS_IRI_HTTPS
	}
}
pub struct OnlineBusinessIriOrLabel;
impl PartialEq<&str> for OnlineBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnlineBusinessIri || *other == ONLINE_BUSINESS_LABEL
	}
}
impl PartialEq<OnlineBusinessIriOrLabel> for &str {
	fn eq(&self, other: &OnlineBusinessIriOrLabel) -> bool {
		*self == OnlineBusinessIri || *self == ONLINE_BUSINESS_LABEL
	}
}
