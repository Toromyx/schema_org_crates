/// <https://schema.org/PublicHealth>
pub const PUBLIC_HEALTH_IRI_HTTP: &str = "http://schema.org/PublicHealth";
/// <https://schema.org/PublicHealth>
pub const PUBLIC_HEALTH_IRI_HTTPS: &str = "https://schema.org/PublicHealth";
/// <https://schema.org/PublicHealth>
pub const PUBLIC_HEALTH_LABEL: &str = "PublicHealth";
pub struct PublicHealthIri;
impl PartialEq<&str> for PublicHealthIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLIC_HEALTH_IRI_HTTP || *other == PUBLIC_HEALTH_IRI_HTTPS
	}
}
impl PartialEq<PublicHealthIri> for &str {
	fn eq(&self, other: &PublicHealthIri) -> bool {
		*self == PUBLIC_HEALTH_IRI_HTTP || *self == PUBLIC_HEALTH_IRI_HTTPS
	}
}
pub struct PublicHealthIriOrLabel;
impl PartialEq<&str> for PublicHealthIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicHealthIri || *other == PUBLIC_HEALTH_LABEL
	}
}
impl PartialEq<PublicHealthIriOrLabel> for &str {
	fn eq(&self, other: &PublicHealthIriOrLabel) -> bool {
		*self == PublicHealthIri || *self == PUBLIC_HEALTH_LABEL
	}
}
