/// <https://schema.org/PublicToilet>
pub const PUBLIC_TOILET_IRI_HTTP: &str = "http://schema.org/PublicToilet";
/// <https://schema.org/PublicToilet>
pub const PUBLIC_TOILET_IRI_HTTPS: &str = "https://schema.org/PublicToilet";
/// <https://schema.org/PublicToilet>
pub const PUBLIC_TOILET_LABEL: &str = "PublicToilet";
pub struct PublicToiletIri;
impl PartialEq<&str> for PublicToiletIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLIC_TOILET_IRI_HTTP || *other == PUBLIC_TOILET_IRI_HTTPS
	}
}
impl PartialEq<PublicToiletIri> for &str {
	fn eq(&self, other: &PublicToiletIri) -> bool {
		*self == PUBLIC_TOILET_IRI_HTTP || *self == PUBLIC_TOILET_IRI_HTTPS
	}
}
pub struct PublicToiletIriOrLabel;
impl PartialEq<&str> for PublicToiletIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublicToiletIri || *other == PUBLIC_TOILET_LABEL
	}
}
impl PartialEq<PublicToiletIriOrLabel> for &str {
	fn eq(&self, other: &PublicToiletIriOrLabel) -> bool {
		*self == PublicToiletIri || *self == PUBLIC_TOILET_LABEL
	}
}
