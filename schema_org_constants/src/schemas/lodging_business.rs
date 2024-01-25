/// <https://schema.org/LodgingBusiness>
pub const LODGING_BUSINESS_IRI_HTTP: &str = "http://schema.org/LodgingBusiness";
/// <https://schema.org/LodgingBusiness>
pub const LODGING_BUSINESS_IRI_HTTPS: &str = "https://schema.org/LodgingBusiness";
/// <https://schema.org/LodgingBusiness>
pub const LODGING_BUSINESS_LABEL: &str = "LodgingBusiness";
pub struct LodgingBusinessIri;
impl PartialEq<&str> for LodgingBusinessIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LODGING_BUSINESS_IRI_HTTP || *other == LODGING_BUSINESS_IRI_HTTPS
	}
}
impl PartialEq<LodgingBusinessIri> for &str {
	fn eq(&self, other: &LodgingBusinessIri) -> bool {
		*self == LODGING_BUSINESS_IRI_HTTP || *self == LODGING_BUSINESS_IRI_HTTPS
	}
}
pub struct LodgingBusinessIriOrLabel;
impl PartialEq<&str> for LodgingBusinessIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LodgingBusinessIri || *other == LODGING_BUSINESS_LABEL
	}
}
impl PartialEq<LodgingBusinessIriOrLabel> for &str {
	fn eq(&self, other: &LodgingBusinessIriOrLabel) -> bool {
		*self == LodgingBusinessIri || *self == LODGING_BUSINESS_LABEL
	}
}
