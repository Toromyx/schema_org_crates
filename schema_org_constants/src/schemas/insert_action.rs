/// <https://schema.org/InsertAction>
pub const INSERT_ACTION_IRI_HTTP: &str = "http://schema.org/InsertAction";
/// <https://schema.org/InsertAction>
pub const INSERT_ACTION_IRI_HTTPS: &str = "https://schema.org/InsertAction";
/// <https://schema.org/InsertAction>
pub const INSERT_ACTION_LABEL: &str = "InsertAction";
pub struct InsertActionIri;
impl PartialEq<&str> for InsertActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSERT_ACTION_IRI_HTTP || *other == INSERT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<InsertActionIri> for &str {
	fn eq(&self, other: &InsertActionIri) -> bool {
		*self == INSERT_ACTION_IRI_HTTP || *self == INSERT_ACTION_IRI_HTTPS
	}
}
pub struct InsertActionIriOrLabel;
impl PartialEq<&str> for InsertActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InsertActionIri || *other == INSERT_ACTION_LABEL
	}
}
impl PartialEq<InsertActionIriOrLabel> for &str {
	fn eq(&self, other: &InsertActionIriOrLabel) -> bool {
		*self == InsertActionIri || *self == INSERT_ACTION_LABEL
	}
}
