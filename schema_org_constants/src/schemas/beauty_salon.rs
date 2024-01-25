/// <https://schema.org/BeautySalon>
pub const BEAUTY_SALON_IRI_HTTP: &str = "http://schema.org/BeautySalon";
/// <https://schema.org/BeautySalon>
pub const BEAUTY_SALON_IRI_HTTPS: &str = "https://schema.org/BeautySalon";
/// <https://schema.org/BeautySalon>
pub const BEAUTY_SALON_LABEL: &str = "BeautySalon";
pub struct BeautySalonIri;
impl PartialEq<&str> for BeautySalonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BEAUTY_SALON_IRI_HTTP || *other == BEAUTY_SALON_IRI_HTTPS
	}
}
impl PartialEq<BeautySalonIri> for &str {
	fn eq(&self, other: &BeautySalonIri) -> bool {
		*self == BEAUTY_SALON_IRI_HTTP || *self == BEAUTY_SALON_IRI_HTTPS
	}
}
pub struct BeautySalonIriOrLabel;
impl PartialEq<&str> for BeautySalonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BeautySalonIri || *other == BEAUTY_SALON_LABEL
	}
}
impl PartialEq<BeautySalonIriOrLabel> for &str {
	fn eq(&self, other: &BeautySalonIriOrLabel) -> bool {
		*self == BeautySalonIri || *self == BEAUTY_SALON_LABEL
	}
}
