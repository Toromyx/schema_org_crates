/// <https://schema.org/CleaningFee>
pub const CLEANING_FEE_IRI_HTTP: &str = "http://schema.org/CleaningFee";
/// <https://schema.org/CleaningFee>
pub const CLEANING_FEE_IRI_HTTPS: &str = "https://schema.org/CleaningFee";
/// <https://schema.org/CleaningFee>
pub const CLEANING_FEE_LABEL: &str = "CleaningFee";
pub struct CleaningFeeIri;
impl PartialEq<&str> for CleaningFeeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLEANING_FEE_IRI_HTTP || *other == CLEANING_FEE_IRI_HTTPS
	}
}
impl PartialEq<CleaningFeeIri> for &str {
	fn eq(&self, other: &CleaningFeeIri) -> bool {
		*self == CLEANING_FEE_IRI_HTTP || *self == CLEANING_FEE_IRI_HTTPS
	}
}
pub struct CleaningFeeIriOrLabel;
impl PartialEq<&str> for CleaningFeeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CleaningFeeIri || *other == CLEANING_FEE_LABEL
	}
}
impl PartialEq<CleaningFeeIriOrLabel> for &str {
	fn eq(&self, other: &CleaningFeeIriOrLabel) -> bool {
		*self == CleaningFeeIri || *self == CLEANING_FEE_LABEL
	}
}
