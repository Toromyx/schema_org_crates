/// <https://schema.org/LegislationObject>
pub const LEGISLATION_OBJECT_IRI_HTTP: &str = "http://schema.org/LegislationObject";
/// <https://schema.org/LegislationObject>
pub const LEGISLATION_OBJECT_IRI_HTTPS: &str = "https://schema.org/LegislationObject";
/// <https://schema.org/LegislationObject>
pub const LEGISLATION_OBJECT_LABEL: &str = "LegislationObject";
pub struct LegislationObjectIri;
impl PartialEq<&str> for LegislationObjectIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEGISLATION_OBJECT_IRI_HTTP || *other == LEGISLATION_OBJECT_IRI_HTTPS
	}
}
impl PartialEq<LegislationObjectIri> for &str {
	fn eq(&self, other: &LegislationObjectIri) -> bool {
		*self == LEGISLATION_OBJECT_IRI_HTTP || *self == LEGISLATION_OBJECT_IRI_HTTPS
	}
}
pub struct LegislationObjectIriOrLabel;
impl PartialEq<&str> for LegislationObjectIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LegislationObjectIri || *other == LEGISLATION_OBJECT_LABEL
	}
}
impl PartialEq<LegislationObjectIriOrLabel> for &str {
	fn eq(&self, other: &LegislationObjectIriOrLabel) -> bool {
		*self == LegislationObjectIri || *self == LEGISLATION_OBJECT_LABEL
	}
}
