/// <https://schema.org/muscleAction>
pub const MUSCLE_ACTION_PROPERTY_IRI_HTTP: &str = "http://schema.org/muscleAction";
/// <https://schema.org/muscleAction>
pub const MUSCLE_ACTION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/muscleAction";
/// <https://schema.org/muscleAction>
pub const MUSCLE_ACTION_PROPERTY_LABEL: &str = "muscleAction";
pub struct MuscleActionPropertyIri;
impl PartialEq<&str> for MuscleActionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSCLE_ACTION_PROPERTY_IRI_HTTP || *other == MUSCLE_ACTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MuscleActionPropertyIri> for &str {
	fn eq(&self, other: &MuscleActionPropertyIri) -> bool {
		*self == MUSCLE_ACTION_PROPERTY_IRI_HTTP || *self == MUSCLE_ACTION_PROPERTY_IRI_HTTPS
	}
}
pub struct MuscleActionPropertyIriOrLabel;
impl PartialEq<&str> for MuscleActionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MuscleActionPropertyIri || *other == MUSCLE_ACTION_PROPERTY_LABEL
	}
}
impl PartialEq<MuscleActionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MuscleActionPropertyIriOrLabel) -> bool {
		*self == MuscleActionPropertyIri || *self == MUSCLE_ACTION_PROPERTY_LABEL
	}
}
