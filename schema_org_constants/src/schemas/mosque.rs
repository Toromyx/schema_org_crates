/// <https://schema.org/Mosque>
pub const MOSQUE_IRI_HTTP: &str = "http://schema.org/Mosque";
/// <https://schema.org/Mosque>
pub const MOSQUE_IRI_HTTPS: &str = "https://schema.org/Mosque";
/// <https://schema.org/Mosque>
pub const MOSQUE_LABEL: &str = "Mosque";
pub struct MosqueIri;
impl PartialEq<&str> for MosqueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOSQUE_IRI_HTTP || *other == MOSQUE_IRI_HTTPS
	}
}
impl PartialEq<MosqueIri> for &str {
	fn eq(&self, other: &MosqueIri) -> bool {
		*self == MOSQUE_IRI_HTTP || *self == MOSQUE_IRI_HTTPS
	}
}
pub struct MosqueIriOrLabel;
impl PartialEq<&str> for MosqueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MosqueIri || *other == MOSQUE_LABEL
	}
}
impl PartialEq<MosqueIriOrLabel> for &str {
	fn eq(&self, other: &MosqueIriOrLabel) -> bool {
		*self == MosqueIri || *self == MOSQUE_LABEL
	}
}
