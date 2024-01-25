/// <https://schema.org/CampingPitch>
pub const CAMPING_PITCH_IRI_HTTP: &str = "http://schema.org/CampingPitch";
/// <https://schema.org/CampingPitch>
pub const CAMPING_PITCH_IRI_HTTPS: &str = "https://schema.org/CampingPitch";
/// <https://schema.org/CampingPitch>
pub const CAMPING_PITCH_LABEL: &str = "CampingPitch";
pub struct CampingPitchIri;
impl PartialEq<&str> for CampingPitchIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAMPING_PITCH_IRI_HTTP || *other == CAMPING_PITCH_IRI_HTTPS
	}
}
impl PartialEq<CampingPitchIri> for &str {
	fn eq(&self, other: &CampingPitchIri) -> bool {
		*self == CAMPING_PITCH_IRI_HTTP || *self == CAMPING_PITCH_IRI_HTTPS
	}
}
pub struct CampingPitchIriOrLabel;
impl PartialEq<&str> for CampingPitchIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CampingPitchIri || *other == CAMPING_PITCH_LABEL
	}
}
impl PartialEq<CampingPitchIriOrLabel> for &str {
	fn eq(&self, other: &CampingPitchIriOrLabel) -> bool {
		*self == CampingPitchIri || *self == CAMPING_PITCH_LABEL
	}
}
