/// <https://schema.org/MathSolver>
pub const MATH_SOLVER_IRI_HTTP: &str = "http://schema.org/MathSolver";
/// <https://schema.org/MathSolver>
pub const MATH_SOLVER_IRI_HTTPS: &str = "https://schema.org/MathSolver";
/// <https://schema.org/MathSolver>
pub const MATH_SOLVER_LABEL: &str = "MathSolver";
pub struct MathSolverIri;
impl PartialEq<&str> for MathSolverIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MATH_SOLVER_IRI_HTTP || *other == MATH_SOLVER_IRI_HTTPS
	}
}
impl PartialEq<MathSolverIri> for &str {
	fn eq(&self, other: &MathSolverIri) -> bool {
		*self == MATH_SOLVER_IRI_HTTP || *self == MATH_SOLVER_IRI_HTTPS
	}
}
pub struct MathSolverIriOrLabel;
impl PartialEq<&str> for MathSolverIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MathSolverIri || *other == MATH_SOLVER_LABEL
	}
}
impl PartialEq<MathSolverIriOrLabel> for &str {
	fn eq(&self, other: &MathSolverIriOrLabel) -> bool {
		*self == MathSolverIri || *self == MATH_SOLVER_LABEL
	}
}
