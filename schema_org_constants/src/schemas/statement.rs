/// <https://schema.org/Statement>
pub const STATEMENT_IRI_HTTP: &str = "http://schema.org/Statement";
/// <https://schema.org/Statement>
pub const STATEMENT_IRI_HTTPS: &str = "https://schema.org/Statement";
/// <https://schema.org/Statement>
pub const STATEMENT_LABEL: &str = "Statement";
pub struct StatementIri;
impl PartialEq<&str> for StatementIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STATEMENT_IRI_HTTP || *other == STATEMENT_IRI_HTTPS
	}
}
impl PartialEq<StatementIri> for &str {
	fn eq(&self, other: &StatementIri) -> bool {
		*self == STATEMENT_IRI_HTTP || *self == STATEMENT_IRI_HTTPS
	}
}
pub struct StatementIriOrLabel;
impl PartialEq<&str> for StatementIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StatementIri || *other == STATEMENT_LABEL
	}
}
impl PartialEq<StatementIriOrLabel> for &str {
	fn eq(&self, other: &StatementIriOrLabel) -> bool {
		*self == StatementIri || *self == STATEMENT_LABEL
	}
}
