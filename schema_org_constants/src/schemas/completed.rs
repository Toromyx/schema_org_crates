/// <https://schema.org/Completed>
pub const COMPLETED_IRI_HTTP: &str = "http://schema.org/Completed";
/// <https://schema.org/Completed>
pub const COMPLETED_IRI_HTTPS: &str = "https://schema.org/Completed";
/// <https://schema.org/Completed>
pub const COMPLETED_LABEL: &str = "Completed";
pub struct CompletedIri;
impl PartialEq<&str> for CompletedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPLETED_IRI_HTTP || *other == COMPLETED_IRI_HTTPS
	}
}
impl PartialEq<CompletedIri> for &str {
	fn eq(&self, other: &CompletedIri) -> bool {
		*self == COMPLETED_IRI_HTTP || *self == COMPLETED_IRI_HTTPS
	}
}
pub struct CompletedIriOrLabel;
impl PartialEq<&str> for CompletedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompletedIri || *other == COMPLETED_LABEL
	}
}
impl PartialEq<CompletedIriOrLabel> for &str {
	fn eq(&self, other: &CompletedIriOrLabel) -> bool {
		*self == CompletedIri || *self == COMPLETED_LABEL
	}
}
