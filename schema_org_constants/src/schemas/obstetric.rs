/// <https://schema.org/Obstetric>
pub const OBSTETRIC_IRI_HTTP: &str = "http://schema.org/Obstetric";
/// <https://schema.org/Obstetric>
pub const OBSTETRIC_IRI_HTTPS: &str = "https://schema.org/Obstetric";
/// <https://schema.org/Obstetric>
pub const OBSTETRIC_LABEL: &str = "Obstetric";
pub struct ObstetricIri;
impl PartialEq<&str> for ObstetricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OBSTETRIC_IRI_HTTP || *other == OBSTETRIC_IRI_HTTPS
	}
}
impl PartialEq<ObstetricIri> for &str {
	fn eq(&self, other: &ObstetricIri) -> bool {
		*self == OBSTETRIC_IRI_HTTP || *self == OBSTETRIC_IRI_HTTPS
	}
}
pub struct ObstetricIriOrLabel;
impl PartialEq<&str> for ObstetricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ObstetricIri || *other == OBSTETRIC_LABEL
	}
}
impl PartialEq<ObstetricIriOrLabel> for &str {
	fn eq(&self, other: &ObstetricIriOrLabel) -> bool {
		*self == ObstetricIri || *self == OBSTETRIC_LABEL
	}
}
