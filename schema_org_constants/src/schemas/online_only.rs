/// <https://schema.org/OnlineOnly>
pub const ONLINE_ONLY_IRI_HTTP: &str = "http://schema.org/OnlineOnly";
/// <https://schema.org/OnlineOnly>
pub const ONLINE_ONLY_IRI_HTTPS: &str = "https://schema.org/OnlineOnly";
/// <https://schema.org/OnlineOnly>
pub const ONLINE_ONLY_LABEL: &str = "OnlineOnly";
pub struct OnlineOnlyIri;
impl PartialEq<&str> for OnlineOnlyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONLINE_ONLY_IRI_HTTP || *other == ONLINE_ONLY_IRI_HTTPS
	}
}
impl PartialEq<OnlineOnlyIri> for &str {
	fn eq(&self, other: &OnlineOnlyIri) -> bool {
		*self == ONLINE_ONLY_IRI_HTTP || *self == ONLINE_ONLY_IRI_HTTPS
	}
}
pub struct OnlineOnlyIriOrLabel;
impl PartialEq<&str> for OnlineOnlyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnlineOnlyIri || *other == ONLINE_ONLY_LABEL
	}
}
impl PartialEq<OnlineOnlyIriOrLabel> for &str {
	fn eq(&self, other: &OnlineOnlyIriOrLabel) -> bool {
		*self == OnlineOnlyIri || *self == ONLINE_ONLY_LABEL
	}
}
