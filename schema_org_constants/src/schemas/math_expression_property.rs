/// <https://schema.org/mathExpression>
pub const MATH_EXPRESSION_PROPERTY_IRI_HTTP: &str = "http://schema.org/mathExpression";
/// <https://schema.org/mathExpression>
pub const MATH_EXPRESSION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/mathExpression";
/// <https://schema.org/mathExpression>
pub const MATH_EXPRESSION_PROPERTY_LABEL: &str = "mathExpression";
pub struct MathExpressionPropertyIri;
impl PartialEq<&str> for MathExpressionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MATH_EXPRESSION_PROPERTY_IRI_HTTP || *other == MATH_EXPRESSION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MathExpressionPropertyIri> for &str {
	fn eq(&self, other: &MathExpressionPropertyIri) -> bool {
		*self == MATH_EXPRESSION_PROPERTY_IRI_HTTP || *self == MATH_EXPRESSION_PROPERTY_IRI_HTTPS
	}
}
pub struct MathExpressionPropertyIriOrLabel;
impl PartialEq<&str> for MathExpressionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MathExpressionPropertyIri || *other == MATH_EXPRESSION_PROPERTY_LABEL
	}
}
impl PartialEq<MathExpressionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MathExpressionPropertyIriOrLabel) -> bool {
		*self == MathExpressionPropertyIri || *self == MATH_EXPRESSION_PROPERTY_LABEL
	}
}
