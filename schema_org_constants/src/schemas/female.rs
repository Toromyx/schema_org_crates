/// <https://schema.org/Female>
pub const FEMALE_IRI_HTTP: &str = "http://schema.org/Female";
/// <https://schema.org/Female>
pub const FEMALE_IRI_HTTPS: &str = "https://schema.org/Female";
/// <https://schema.org/Female>
pub const FEMALE_LABEL: &str = "Female";
pub struct FemaleIri;
impl PartialEq<&str> for FemaleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FEMALE_IRI_HTTP || *other == FEMALE_IRI_HTTPS
	}
}
impl PartialEq<FemaleIri> for &str {
	fn eq(&self, other: &FemaleIri) -> bool {
		*self == FEMALE_IRI_HTTP || *self == FEMALE_IRI_HTTPS
	}
}
pub struct FemaleIriOrLabel;
impl PartialEq<&str> for FemaleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FemaleIri || *other == FEMALE_LABEL
	}
}
impl PartialEq<FemaleIriOrLabel> for &str {
	fn eq(&self, other: &FemaleIriOrLabel) -> bool {
		*self == FemaleIri || *self == FEMALE_LABEL
	}
}
