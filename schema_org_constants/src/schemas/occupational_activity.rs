/// <https://schema.org/OccupationalActivity>
pub const OCCUPATIONAL_ACTIVITY_IRI_HTTP: &str = "http://schema.org/OccupationalActivity";
/// <https://schema.org/OccupationalActivity>
pub const OCCUPATIONAL_ACTIVITY_IRI_HTTPS: &str = "https://schema.org/OccupationalActivity";
/// <https://schema.org/OccupationalActivity>
pub const OCCUPATIONAL_ACTIVITY_LABEL: &str = "OccupationalActivity";
pub struct OccupationalActivityIri;
impl PartialEq<&str> for OccupationalActivityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OCCUPATIONAL_ACTIVITY_IRI_HTTP || *other == OCCUPATIONAL_ACTIVITY_IRI_HTTPS
	}
}
impl PartialEq<OccupationalActivityIri> for &str {
	fn eq(&self, other: &OccupationalActivityIri) -> bool {
		*self == OCCUPATIONAL_ACTIVITY_IRI_HTTP || *self == OCCUPATIONAL_ACTIVITY_IRI_HTTPS
	}
}
pub struct OccupationalActivityIriOrLabel;
impl PartialEq<&str> for OccupationalActivityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OccupationalActivityIri || *other == OCCUPATIONAL_ACTIVITY_LABEL
	}
}
impl PartialEq<OccupationalActivityIriOrLabel> for &str {
	fn eq(&self, other: &OccupationalActivityIriOrLabel) -> bool {
		*self == OccupationalActivityIri || *self == OCCUPATIONAL_ACTIVITY_LABEL
	}
}
