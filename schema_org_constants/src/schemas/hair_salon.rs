/// <https://schema.org/HairSalon>
pub const HAIR_SALON_IRI_HTTP: &str = "http://schema.org/HairSalon";
/// <https://schema.org/HairSalon>
pub const HAIR_SALON_IRI_HTTPS: &str = "https://schema.org/HairSalon";
/// <https://schema.org/HairSalon>
pub const HAIR_SALON_LABEL: &str = "HairSalon";
pub struct HairSalonIri;
impl PartialEq<&str> for HairSalonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAIR_SALON_IRI_HTTP || *other == HAIR_SALON_IRI_HTTPS
	}
}
impl PartialEq<HairSalonIri> for &str {
	fn eq(&self, other: &HairSalonIri) -> bool {
		*self == HAIR_SALON_IRI_HTTP || *self == HAIR_SALON_IRI_HTTPS
	}
}
pub struct HairSalonIriOrLabel;
impl PartialEq<&str> for HairSalonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HairSalonIri || *other == HAIR_SALON_LABEL
	}
}
impl PartialEq<HairSalonIriOrLabel> for &str {
	fn eq(&self, other: &HairSalonIriOrLabel) -> bool {
		*self == HairSalonIri || *self == HAIR_SALON_LABEL
	}
}
