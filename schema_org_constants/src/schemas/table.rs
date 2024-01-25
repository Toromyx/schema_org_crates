/// <https://schema.org/Table>
pub const TABLE_IRI_HTTP: &str = "http://schema.org/Table";
/// <https://schema.org/Table>
pub const TABLE_IRI_HTTPS: &str = "https://schema.org/Table";
/// <https://schema.org/Table>
pub const TABLE_LABEL: &str = "Table";
pub struct TableIri;
impl PartialEq<&str> for TableIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TABLE_IRI_HTTP || *other == TABLE_IRI_HTTPS
	}
}
impl PartialEq<TableIri> for &str {
	fn eq(&self, other: &TableIri) -> bool {
		*self == TABLE_IRI_HTTP || *self == TABLE_IRI_HTTPS
	}
}
pub struct TableIriOrLabel;
impl PartialEq<&str> for TableIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TableIri || *other == TABLE_LABEL
	}
}
impl PartialEq<TableIriOrLabel> for &str {
	fn eq(&self, other: &TableIriOrLabel) -> bool {
		*self == TableIri || *self == TABLE_LABEL
	}
}
