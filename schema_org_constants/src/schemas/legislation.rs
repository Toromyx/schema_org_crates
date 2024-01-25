/// <https://schema.org/Legislation>
pub const LEGISLATION_IRI_HTTP: &str = "http://schema.org/Legislation";
/// <https://schema.org/Legislation>
pub const LEGISLATION_IRI_HTTPS: &str = "https://schema.org/Legislation";
/// <https://schema.org/Legislation>
pub const LEGISLATION_LABEL: &str = "Legislation";
pub struct LegislationIri;
impl PartialEq<&str> for LegislationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_IRI_HTTP || *other == LEGISLATION_IRI_HTTPS
	}
}
impl PartialEq<LegislationIri> for &str {
	fn eq(&self, other: &LegislationIri) -> bool {
		*self == LEGISLATION_IRI_HTTP || *self == LEGISLATION_IRI_HTTPS
	}
}
pub struct LegislationIriOrLabel;
impl PartialEq<&str> for LegislationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationIri || *other == LEGISLATION_LABEL
	}
}
impl PartialEq<LegislationIriOrLabel> for &str {
	fn eq(&self, other: &LegislationIriOrLabel) -> bool {
		*self == LegislationIri || *self == LEGISLATION_LABEL
	}
}
