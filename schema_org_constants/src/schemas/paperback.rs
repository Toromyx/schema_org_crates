/// <https://schema.org/Paperback>
pub const PAPERBACK_IRI_HTTP: &str = "http://schema.org/Paperback";
/// <https://schema.org/Paperback>
pub const PAPERBACK_IRI_HTTPS: &str = "https://schema.org/Paperback";
/// <https://schema.org/Paperback>
pub const PAPERBACK_LABEL: &str = "Paperback";
pub struct PaperbackIri;
impl PartialEq<&str> for PaperbackIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAPERBACK_IRI_HTTP || *other == PAPERBACK_IRI_HTTPS
	}
}
impl PartialEq<PaperbackIri> for &str {
	fn eq(&self, other: &PaperbackIri) -> bool {
		*self == PAPERBACK_IRI_HTTP || *self == PAPERBACK_IRI_HTTPS
	}
}
pub struct PaperbackIriOrLabel;
impl PartialEq<&str> for PaperbackIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PaperbackIri || *other == PAPERBACK_LABEL
	}
}
impl PartialEq<PaperbackIriOrLabel> for &str {
	fn eq(&self, other: &PaperbackIriOrLabel) -> bool {
		*self == PaperbackIri || *self == PAPERBACK_LABEL
	}
}
