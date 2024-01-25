/// <https://schema.org/NailSalon>
pub const NAIL_SALON_IRI_HTTP: &str = "http://schema.org/NailSalon";
/// <https://schema.org/NailSalon>
pub const NAIL_SALON_IRI_HTTPS: &str = "https://schema.org/NailSalon";
/// <https://schema.org/NailSalon>
pub const NAIL_SALON_LABEL: &str = "NailSalon";
pub struct NailSalonIri;
impl PartialEq<&str> for NailSalonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NAIL_SALON_IRI_HTTP || *other == NAIL_SALON_IRI_HTTPS
	}
}
impl PartialEq<NailSalonIri> for &str {
	fn eq(&self, other: &NailSalonIri) -> bool {
		*self == NAIL_SALON_IRI_HTTP || *self == NAIL_SALON_IRI_HTTPS
	}
}
pub struct NailSalonIriOrLabel;
impl PartialEq<&str> for NailSalonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NailSalonIri || *other == NAIL_SALON_LABEL
	}
}
impl PartialEq<NailSalonIriOrLabel> for &str {
	fn eq(&self, other: &NailSalonIriOrLabel) -> bool {
		*self == NailSalonIri || *self == NAIL_SALON_LABEL
	}
}
