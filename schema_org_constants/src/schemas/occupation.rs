/// <https://schema.org/Occupation>
pub const OCCUPATION_IRI_HTTP: &str = "http://schema.org/Occupation";
/// <https://schema.org/Occupation>
pub const OCCUPATION_IRI_HTTPS: &str = "https://schema.org/Occupation";
/// <https://schema.org/Occupation>
pub const OCCUPATION_LABEL: &str = "Occupation";
pub struct OccupationIri;
impl PartialEq<&str> for OccupationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATION_IRI_HTTP || *other == OCCUPATION_IRI_HTTPS
	}
}
impl PartialEq<OccupationIri> for &str {
	fn eq(&self, other: &OccupationIri) -> bool {
		*self == OCCUPATION_IRI_HTTP || *self == OCCUPATION_IRI_HTTPS
	}
}
pub struct OccupationIriOrLabel;
impl PartialEq<&str> for OccupationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationIri || *other == OCCUPATION_LABEL
	}
}
impl PartialEq<OccupationIriOrLabel> for &str {
	fn eq(&self, other: &OccupationIriOrLabel) -> bool {
		*self == OccupationIri || *self == OCCUPATION_LABEL
	}
}
