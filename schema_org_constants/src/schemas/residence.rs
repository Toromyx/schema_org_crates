/// <https://schema.org/Residence>
pub const RESIDENCE_IRI_HTTP: &str = "http://schema.org/Residence";
/// <https://schema.org/Residence>
pub const RESIDENCE_IRI_HTTPS: &str = "https://schema.org/Residence";
/// <https://schema.org/Residence>
pub const RESIDENCE_LABEL: &str = "Residence";
pub struct ResidenceIri;
impl PartialEq<&str> for ResidenceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESIDENCE_IRI_HTTP || *other == RESIDENCE_IRI_HTTPS
	}
}
impl PartialEq<ResidenceIri> for &str {
	fn eq(&self, other: &ResidenceIri) -> bool {
		*self == RESIDENCE_IRI_HTTP || *self == RESIDENCE_IRI_HTTPS
	}
}
pub struct ResidenceIriOrLabel;
impl PartialEq<&str> for ResidenceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResidenceIri || *other == RESIDENCE_LABEL
	}
}
impl PartialEq<ResidenceIriOrLabel> for &str {
	fn eq(&self, other: &ResidenceIriOrLabel) -> bool {
		*self == ResidenceIri || *self == RESIDENCE_LABEL
	}
}
