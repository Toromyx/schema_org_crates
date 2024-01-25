/// <https://schema.org/Distillery>
pub const DISTILLERY_IRI_HTTP: &str = "http://schema.org/Distillery";
/// <https://schema.org/Distillery>
pub const DISTILLERY_IRI_HTTPS: &str = "https://schema.org/Distillery";
/// <https://schema.org/Distillery>
pub const DISTILLERY_LABEL: &str = "Distillery";
pub struct DistilleryIri;
impl PartialEq<&str> for DistilleryIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DISTILLERY_IRI_HTTP || *other == DISTILLERY_IRI_HTTPS
	}
}
impl PartialEq<DistilleryIri> for &str {
	fn eq(&self, other: &DistilleryIri) -> bool {
		*self == DISTILLERY_IRI_HTTP || *self == DISTILLERY_IRI_HTTPS
	}
}
pub struct DistilleryIriOrLabel;
impl PartialEq<&str> for DistilleryIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DistilleryIri || *other == DISTILLERY_LABEL
	}
}
impl PartialEq<DistilleryIriOrLabel> for &str {
	fn eq(&self, other: &DistilleryIriOrLabel) -> bool {
		*self == DistilleryIri || *self == DISTILLERY_LABEL
	}
}
