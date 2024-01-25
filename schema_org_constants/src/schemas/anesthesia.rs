/// <https://schema.org/Anesthesia>
pub const ANESTHESIA_IRI_HTTP: &str = "http://schema.org/Anesthesia";
/// <https://schema.org/Anesthesia>
pub const ANESTHESIA_IRI_HTTPS: &str = "https://schema.org/Anesthesia";
/// <https://schema.org/Anesthesia>
pub const ANESTHESIA_LABEL: &str = "Anesthesia";
pub struct AnesthesiaIri;
impl PartialEq<&str> for AnesthesiaIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ANESTHESIA_IRI_HTTP || *other == ANESTHESIA_IRI_HTTPS
	}
}
impl PartialEq<AnesthesiaIri> for &str {
	fn eq(&self, other: &AnesthesiaIri) -> bool {
		*self == ANESTHESIA_IRI_HTTP || *self == ANESTHESIA_IRI_HTTPS
	}
}
pub struct AnesthesiaIriOrLabel;
impl PartialEq<&str> for AnesthesiaIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AnesthesiaIri || *other == ANESTHESIA_LABEL
	}
}
impl PartialEq<AnesthesiaIriOrLabel> for &str {
	fn eq(&self, other: &AnesthesiaIriOrLabel) -> bool {
		*self == AnesthesiaIri || *self == ANESTHESIA_LABEL
	}
}
