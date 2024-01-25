/// <https://schema.org/Online>
pub const ONLINE_IRI_HTTP: &str = "http://schema.org/Online";
/// <https://schema.org/Online>
pub const ONLINE_IRI_HTTPS: &str = "https://schema.org/Online";
/// <https://schema.org/Online>
pub const ONLINE_LABEL: &str = "Online";
pub struct OnlineIri;
impl PartialEq<&str> for OnlineIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ONLINE_IRI_HTTP || *other == ONLINE_IRI_HTTPS
	}
}
impl PartialEq<OnlineIri> for &str {
	fn eq(&self, other: &OnlineIri) -> bool {
		*self == ONLINE_IRI_HTTP || *self == ONLINE_IRI_HTTPS
	}
}
pub struct OnlineIriOrLabel;
impl PartialEq<&str> for OnlineIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OnlineIri || *other == ONLINE_LABEL
	}
}
impl PartialEq<OnlineIriOrLabel> for &str {
	fn eq(&self, other: &OnlineIriOrLabel) -> bool {
		*self == OnlineIri || *self == ONLINE_LABEL
	}
}
