/// <https://schema.org/Male>
pub const MALE_IRI_HTTP: &str = "http://schema.org/Male";
/// <https://schema.org/Male>
pub const MALE_IRI_HTTPS: &str = "https://schema.org/Male";
/// <https://schema.org/Male>
pub const MALE_LABEL: &str = "Male";
pub struct MaleIri;
impl PartialEq<&str> for MaleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MALE_IRI_HTTP || *other == MALE_IRI_HTTPS
	}
}
impl PartialEq<MaleIri> for &str {
	fn eq(&self, other: &MaleIri) -> bool {
		*self == MALE_IRI_HTTP || *self == MALE_IRI_HTTPS
	}
}
pub struct MaleIriOrLabel;
impl PartialEq<&str> for MaleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaleIri || *other == MALE_LABEL
	}
}
impl PartialEq<MaleIriOrLabel> for &str {
	fn eq(&self, other: &MaleIriOrLabel) -> bool {
		*self == MaleIri || *self == MALE_LABEL
	}
}
