/// <https://schema.org/CatholicChurch>
pub const CATHOLIC_CHURCH_IRI_HTTP: &str = "http://schema.org/CatholicChurch";
/// <https://schema.org/CatholicChurch>
pub const CATHOLIC_CHURCH_IRI_HTTPS: &str = "https://schema.org/CatholicChurch";
/// <https://schema.org/CatholicChurch>
pub const CATHOLIC_CHURCH_LABEL: &str = "CatholicChurch";
pub struct CatholicChurchIri;
impl PartialEq<&str> for CatholicChurchIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CATHOLIC_CHURCH_IRI_HTTP || *other == CATHOLIC_CHURCH_IRI_HTTPS
	}
}
impl PartialEq<CatholicChurchIri> for &str {
	fn eq(&self, other: &CatholicChurchIri) -> bool {
		*self == CATHOLIC_CHURCH_IRI_HTTP || *self == CATHOLIC_CHURCH_IRI_HTTPS
	}
}
pub struct CatholicChurchIriOrLabel;
impl PartialEq<&str> for CatholicChurchIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CatholicChurchIri || *other == CATHOLIC_CHURCH_LABEL
	}
}
impl PartialEq<CatholicChurchIriOrLabel> for &str {
	fn eq(&self, other: &CatholicChurchIriOrLabel) -> bool {
		*self == CatholicChurchIri || *self == CATHOLIC_CHURCH_LABEL
	}
}
