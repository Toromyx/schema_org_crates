/// <https://schema.org/Preschool>
pub const PRESCHOOL_IRI_HTTP: &str = "http://schema.org/Preschool";
/// <https://schema.org/Preschool>
pub const PRESCHOOL_IRI_HTTPS: &str = "https://schema.org/Preschool";
/// <https://schema.org/Preschool>
pub const PRESCHOOL_LABEL: &str = "Preschool";
pub struct PreschoolIri;
impl PartialEq<&str> for PreschoolIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRESCHOOL_IRI_HTTP || *other == PRESCHOOL_IRI_HTTPS
	}
}
impl PartialEq<PreschoolIri> for &str {
	fn eq(&self, other: &PreschoolIri) -> bool {
		*self == PRESCHOOL_IRI_HTTP || *self == PRESCHOOL_IRI_HTTPS
	}
}
pub struct PreschoolIriOrLabel;
impl PartialEq<&str> for PreschoolIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreschoolIri || *other == PRESCHOOL_LABEL
	}
}
impl PartialEq<PreschoolIriOrLabel> for &str {
	fn eq(&self, other: &PreschoolIriOrLabel) -> bool {
		*self == PreschoolIri || *self == PRESCHOOL_LABEL
	}
}
