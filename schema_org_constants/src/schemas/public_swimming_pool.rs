/// <https://schema.org/PublicSwimmingPool>
pub const PUBLIC_SWIMMING_POOL_IRI_HTTP: &str = "http://schema.org/PublicSwimmingPool";
/// <https://schema.org/PublicSwimmingPool>
pub const PUBLIC_SWIMMING_POOL_IRI_HTTPS: &str = "https://schema.org/PublicSwimmingPool";
/// <https://schema.org/PublicSwimmingPool>
pub const PUBLIC_SWIMMING_POOL_LABEL: &str = "PublicSwimmingPool";
pub struct PublicSwimmingPoolIri;
impl PartialEq<&str> for PublicSwimmingPoolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLIC_SWIMMING_POOL_IRI_HTTP || *other == PUBLIC_SWIMMING_POOL_IRI_HTTPS
	}
}
impl PartialEq<PublicSwimmingPoolIri> for &str {
	fn eq(&self, other: &PublicSwimmingPoolIri) -> bool {
		*self == PUBLIC_SWIMMING_POOL_IRI_HTTP || *self == PUBLIC_SWIMMING_POOL_IRI_HTTPS
	}
}
pub struct PublicSwimmingPoolIriOrLabel;
impl PartialEq<&str> for PublicSwimmingPoolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicSwimmingPoolIri || *other == PUBLIC_SWIMMING_POOL_LABEL
	}
}
impl PartialEq<PublicSwimmingPoolIriOrLabel> for &str {
	fn eq(&self, other: &PublicSwimmingPoolIriOrLabel) -> bool {
		*self == PublicSwimmingPoolIri || *self == PUBLIC_SWIMMING_POOL_LABEL
	}
}
