/// <https://schema.org/Mass>
pub const MASS_IRI_HTTP: &str = "http://schema.org/Mass";
/// <https://schema.org/Mass>
pub const MASS_IRI_HTTPS: &str = "https://schema.org/Mass";
/// <https://schema.org/Mass>
pub const MASS_LABEL: &str = "Mass";
pub struct MassIri;
impl PartialEq<&str> for MassIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MASS_IRI_HTTP || *other == MASS_IRI_HTTPS
	}
}
impl PartialEq<MassIri> for &str {
	fn eq(&self, other: &MassIri) -> bool {
		*self == MASS_IRI_HTTP || *self == MASS_IRI_HTTPS
	}
}
pub struct MassIriOrLabel;
impl PartialEq<&str> for MassIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MassIri || *other == MASS_LABEL
	}
}
impl PartialEq<MassIriOrLabel> for &str {
	fn eq(&self, other: &MassIriOrLabel) -> bool {
		*self == MassIri || *self == MASS_LABEL
	}
}
