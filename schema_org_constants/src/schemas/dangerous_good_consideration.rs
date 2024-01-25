/// <https://schema.org/DangerousGoodConsideration>
pub const DANGEROUS_GOOD_CONSIDERATION_IRI_HTTP: &str =
	"http://schema.org/DangerousGoodConsideration";
/// <https://schema.org/DangerousGoodConsideration>
pub const DANGEROUS_GOOD_CONSIDERATION_IRI_HTTPS: &str =
	"https://schema.org/DangerousGoodConsideration";
/// <https://schema.org/DangerousGoodConsideration>
pub const DANGEROUS_GOOD_CONSIDERATION_LABEL: &str = "DangerousGoodConsideration";
pub struct DangerousGoodConsiderationIri;
impl PartialEq<&str> for DangerousGoodConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DANGEROUS_GOOD_CONSIDERATION_IRI_HTTP
			|| *other == DANGEROUS_GOOD_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<DangerousGoodConsiderationIri> for &str {
	fn eq(&self, other: &DangerousGoodConsiderationIri) -> bool {
		*self == DANGEROUS_GOOD_CONSIDERATION_IRI_HTTP
			|| *self == DANGEROUS_GOOD_CONSIDERATION_IRI_HTTPS
	}
}
pub struct DangerousGoodConsiderationIriOrLabel;
impl PartialEq<&str> for DangerousGoodConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DangerousGoodConsiderationIri || *other == DANGEROUS_GOOD_CONSIDERATION_LABEL
	}
}
impl PartialEq<DangerousGoodConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &DangerousGoodConsiderationIriOrLabel) -> bool {
		*self == DangerousGoodConsiderationIri || *self == DANGEROUS_GOOD_CONSIDERATION_LABEL
	}
}
