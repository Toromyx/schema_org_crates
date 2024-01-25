/// <https://schema.org/OnlineFull>
pub const ONLINE_FULL_IRI_HTTP: &str = "http://schema.org/OnlineFull";
/// <https://schema.org/OnlineFull>
pub const ONLINE_FULL_IRI_HTTPS: &str = "https://schema.org/OnlineFull";
/// <https://schema.org/OnlineFull>
pub const ONLINE_FULL_LABEL: &str = "OnlineFull";
pub struct OnlineFullIri;
impl PartialEq<&str> for OnlineFullIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONLINE_FULL_IRI_HTTP || *other == ONLINE_FULL_IRI_HTTPS
	}
}
impl PartialEq<OnlineFullIri> for &str {
	fn eq(&self, other: &OnlineFullIri) -> bool {
		*self == ONLINE_FULL_IRI_HTTP || *self == ONLINE_FULL_IRI_HTTPS
	}
}
pub struct OnlineFullIriOrLabel;
impl PartialEq<&str> for OnlineFullIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnlineFullIri || *other == ONLINE_FULL_LABEL
	}
}
impl PartialEq<OnlineFullIriOrLabel> for &str {
	fn eq(&self, other: &OnlineFullIriOrLabel) -> bool {
		*self == OnlineFullIri || *self == ONLINE_FULL_LABEL
	}
}
