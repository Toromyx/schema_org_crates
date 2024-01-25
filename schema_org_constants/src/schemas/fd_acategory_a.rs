/// <https://schema.org/FDAcategoryA>
pub const FD_ACATEGORY_A_IRI_HTTP: &str = "http://schema.org/FDAcategoryA";
/// <https://schema.org/FDAcategoryA>
pub const FD_ACATEGORY_A_IRI_HTTPS: &str = "https://schema.org/FDAcategoryA";
/// <https://schema.org/FDAcategoryA>
pub const FD_ACATEGORY_A_LABEL: &str = "FDAcategoryA";
pub struct FdAcategoryAIri;
impl PartialEq<&str> for FdAcategoryAIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FD_ACATEGORY_A_IRI_HTTP || *other == FD_ACATEGORY_A_IRI_HTTPS
	}
}
impl PartialEq<FdAcategoryAIri> for &str {
	fn eq(&self, other: &FdAcategoryAIri) -> bool {
		*self == FD_ACATEGORY_A_IRI_HTTP || *self == FD_ACATEGORY_A_IRI_HTTPS
	}
}
pub struct FdAcategoryAIriOrLabel;
impl PartialEq<&str> for FdAcategoryAIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FdAcategoryAIri || *other == FD_ACATEGORY_A_LABEL
	}
}
impl PartialEq<FdAcategoryAIriOrLabel> for &str {
	fn eq(&self, other: &FdAcategoryAIriOrLabel) -> bool {
		*self == FdAcategoryAIri || *self == FD_ACATEGORY_A_LABEL
	}
}
