/// <https://schema.org/Midwifery>
pub const MIDWIFERY_IRI_HTTP: &str = "http://schema.org/Midwifery";
/// <https://schema.org/Midwifery>
pub const MIDWIFERY_IRI_HTTPS: &str = "https://schema.org/Midwifery";
/// <https://schema.org/Midwifery>
pub const MIDWIFERY_LABEL: &str = "Midwifery";
pub struct MidwiferyIri;
impl PartialEq<&str> for MidwiferyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MIDWIFERY_IRI_HTTP || *other == MIDWIFERY_IRI_HTTPS
	}
}
impl PartialEq<MidwiferyIri> for &str {
	fn eq(&self, other: &MidwiferyIri) -> bool {
		*self == MIDWIFERY_IRI_HTTP || *self == MIDWIFERY_IRI_HTTPS
	}
}
pub struct MidwiferyIriOrLabel;
impl PartialEq<&str> for MidwiferyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MidwiferyIri || *other == MIDWIFERY_LABEL
	}
}
impl PartialEq<MidwiferyIriOrLabel> for &str {
	fn eq(&self, other: &MidwiferyIriOrLabel) -> bool {
		*self == MidwiferyIri || *self == MIDWIFERY_LABEL
	}
}
