/// <https://schema.org/Chiropractic>
pub const CHIROPRACTIC_IRI_HTTP: &str = "http://schema.org/Chiropractic";
/// <https://schema.org/Chiropractic>
pub const CHIROPRACTIC_IRI_HTTPS: &str = "https://schema.org/Chiropractic";
/// <https://schema.org/Chiropractic>
pub const CHIROPRACTIC_LABEL: &str = "Chiropractic";
pub struct ChiropracticIri;
impl PartialEq<&str> for ChiropracticIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHIROPRACTIC_IRI_HTTP || *other == CHIROPRACTIC_IRI_HTTPS
	}
}
impl PartialEq<ChiropracticIri> for &str {
	fn eq(&self, other: &ChiropracticIri) -> bool {
		*self == CHIROPRACTIC_IRI_HTTP || *self == CHIROPRACTIC_IRI_HTTPS
	}
}
pub struct ChiropracticIriOrLabel;
impl PartialEq<&str> for ChiropracticIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChiropracticIri || *other == CHIROPRACTIC_LABEL
	}
}
impl PartialEq<ChiropracticIriOrLabel> for &str {
	fn eq(&self, other: &ChiropracticIriOrLabel) -> bool {
		*self == ChiropracticIri || *self == CHIROPRACTIC_LABEL
	}
}
