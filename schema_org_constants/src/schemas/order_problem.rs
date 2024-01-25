/// <https://schema.org/OrderProblem>
pub const ORDER_PROBLEM_IRI_HTTP: &str = "http://schema.org/OrderProblem";
/// <https://schema.org/OrderProblem>
pub const ORDER_PROBLEM_IRI_HTTPS: &str = "https://schema.org/OrderProblem";
/// <https://schema.org/OrderProblem>
pub const ORDER_PROBLEM_LABEL: &str = "OrderProblem";
pub struct OrderProblemIri;
impl PartialEq<&str> for OrderProblemIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORDER_PROBLEM_IRI_HTTP || *other == ORDER_PROBLEM_IRI_HTTPS
	}
}
impl PartialEq<OrderProblemIri> for &str {
	fn eq(&self, other: &OrderProblemIri) -> bool {
		*self == ORDER_PROBLEM_IRI_HTTP || *self == ORDER_PROBLEM_IRI_HTTPS
	}
}
pub struct OrderProblemIriOrLabel;
impl PartialEq<&str> for OrderProblemIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OrderProblemIri || *other == ORDER_PROBLEM_LABEL
	}
}
impl PartialEq<OrderProblemIriOrLabel> for &str {
	fn eq(&self, other: &OrderProblemIriOrLabel) -> bool {
		*self == OrderProblemIri || *self == ORDER_PROBLEM_LABEL
	}
}
