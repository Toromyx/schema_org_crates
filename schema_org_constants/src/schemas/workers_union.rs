/// <https://schema.org/WorkersUnion>
pub const WORKERS_UNION_IRI_HTTP: &str = "http://schema.org/WorkersUnion";
/// <https://schema.org/WorkersUnion>
pub const WORKERS_UNION_IRI_HTTPS: &str = "https://schema.org/WorkersUnion";
/// <https://schema.org/WorkersUnion>
pub const WORKERS_UNION_LABEL: &str = "WorkersUnion";
pub struct WorkersUnionIri;
impl PartialEq<&str> for WorkersUnionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == WORKERS_UNION_IRI_HTTP || *other == WORKERS_UNION_IRI_HTTPS
	}
}
impl PartialEq<WorkersUnionIri> for &str {
	fn eq(&self, other: &WorkersUnionIri) -> bool {
		*self == WORKERS_UNION_IRI_HTTP || *self == WORKERS_UNION_IRI_HTTPS
	}
}
pub struct WorkersUnionIriOrLabel;
impl PartialEq<&str> for WorkersUnionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == WorkersUnionIri || *other == WORKERS_UNION_LABEL
	}
}
impl PartialEq<WorkersUnionIriOrLabel> for &str {
	fn eq(&self, other: &WorkersUnionIriOrLabel) -> bool {
		*self == WorkersUnionIri || *self == WORKERS_UNION_LABEL
	}
}
