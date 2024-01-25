/// <https://schema.org/Muscle>
pub const MUSCLE_IRI_HTTP: &str = "http://schema.org/Muscle";
/// <https://schema.org/Muscle>
pub const MUSCLE_IRI_HTTPS: &str = "https://schema.org/Muscle";
/// <https://schema.org/Muscle>
pub const MUSCLE_LABEL: &str = "Muscle";
pub struct MuscleIri;
impl PartialEq<&str> for MuscleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSCLE_IRI_HTTP || *other == MUSCLE_IRI_HTTPS
	}
}
impl PartialEq<MuscleIri> for &str {
	fn eq(&self, other: &MuscleIri) -> bool {
		*self == MUSCLE_IRI_HTTP || *self == MUSCLE_IRI_HTTPS
	}
}
pub struct MuscleIriOrLabel;
impl PartialEq<&str> for MuscleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MuscleIri || *other == MUSCLE_LABEL
	}
}
impl PartialEq<MuscleIriOrLabel> for &str {
	fn eq(&self, other: &MuscleIriOrLabel) -> bool {
		*self == MuscleIri || *self == MUSCLE_LABEL
	}
}
