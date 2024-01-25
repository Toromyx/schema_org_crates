/// <https://schema.org/VeterinaryCare>
pub const VETERINARY_CARE_IRI_HTTP: &str = "http://schema.org/VeterinaryCare";
/// <https://schema.org/VeterinaryCare>
pub const VETERINARY_CARE_IRI_HTTPS: &str = "https://schema.org/VeterinaryCare";
/// <https://schema.org/VeterinaryCare>
pub const VETERINARY_CARE_LABEL: &str = "VeterinaryCare";
pub struct VeterinaryCareIri;
impl PartialEq<&str> for VeterinaryCareIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VETERINARY_CARE_IRI_HTTP || *other == VETERINARY_CARE_IRI_HTTPS
	}
}
impl PartialEq<VeterinaryCareIri> for &str {
	fn eq(&self, other: &VeterinaryCareIri) -> bool {
		*self == VETERINARY_CARE_IRI_HTTP || *self == VETERINARY_CARE_IRI_HTTPS
	}
}
pub struct VeterinaryCareIriOrLabel;
impl PartialEq<&str> for VeterinaryCareIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VeterinaryCareIri || *other == VETERINARY_CARE_LABEL
	}
}
impl PartialEq<VeterinaryCareIriOrLabel> for &str {
	fn eq(&self, other: &VeterinaryCareIriOrLabel) -> bool {
		*self == VeterinaryCareIri || *self == VETERINARY_CARE_LABEL
	}
}
