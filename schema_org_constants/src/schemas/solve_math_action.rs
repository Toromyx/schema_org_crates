/// <https://schema.org/SolveMathAction>
pub const SOLVE_MATH_ACTION_IRI_HTTP: &str = "http://schema.org/SolveMathAction";
/// <https://schema.org/SolveMathAction>
pub const SOLVE_MATH_ACTION_IRI_HTTPS: &str = "https://schema.org/SolveMathAction";
/// <https://schema.org/SolveMathAction>
pub const SOLVE_MATH_ACTION_LABEL: &str = "SolveMathAction";
pub struct SolveMathActionIri;
impl PartialEq<&str> for SolveMathActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SOLVE_MATH_ACTION_IRI_HTTP || *other == SOLVE_MATH_ACTION_IRI_HTTPS
	}
}
impl PartialEq<SolveMathActionIri> for &str {
	fn eq(&self, other: &SolveMathActionIri) -> bool {
		*self == SOLVE_MATH_ACTION_IRI_HTTP || *self == SOLVE_MATH_ACTION_IRI_HTTPS
	}
}
pub struct SolveMathActionIriOrLabel;
impl PartialEq<&str> for SolveMathActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SolveMathActionIri || *other == SOLVE_MATH_ACTION_LABEL
	}
}
impl PartialEq<SolveMathActionIriOrLabel> for &str {
	fn eq(&self, other: &SolveMathActionIriOrLabel) -> bool {
		*self == SolveMathActionIri || *self == SOLVE_MATH_ACTION_LABEL
	}
}
