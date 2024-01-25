/// <https://schema.org/Ligament>
pub const LIGAMENT_IRI_HTTP: &str = "http://schema.org/Ligament";
/// <https://schema.org/Ligament>
pub const LIGAMENT_IRI_HTTPS: &str = "https://schema.org/Ligament";
/// <https://schema.org/Ligament>
pub const LIGAMENT_LABEL: &str = "Ligament";
pub struct LigamentIri;
impl PartialEq<&str> for LigamentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LIGAMENT_IRI_HTTP || *other == LIGAMENT_IRI_HTTPS
	}
}
impl PartialEq<LigamentIri> for &str {
	fn eq(&self, other: &LigamentIri) -> bool {
		*self == LIGAMENT_IRI_HTTP || *self == LIGAMENT_IRI_HTTPS
	}
}
pub struct LigamentIriOrLabel;
impl PartialEq<&str> for LigamentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LigamentIri || *other == LIGAMENT_LABEL
	}
}
impl PartialEq<LigamentIriOrLabel> for &str {
	fn eq(&self, other: &LigamentIriOrLabel) -> bool {
		*self == LigamentIri || *self == LIGAMENT_LABEL
	}
}
