/// <https://schema.org/FDAcategoryB>
pub const FD_ACATEGORY_B_IRI_HTTP: &str = "http://schema.org/FDAcategoryB";
/// <https://schema.org/FDAcategoryB>
pub const FD_ACATEGORY_B_IRI_HTTPS: &str = "https://schema.org/FDAcategoryB";
/// <https://schema.org/FDAcategoryB>
pub const FD_ACATEGORY_B_LABEL: &str = "FDAcategoryB";
pub struct FdAcategoryBIri;
impl PartialEq<&str> for FdAcategoryBIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FD_ACATEGORY_B_IRI_HTTP || *other == FD_ACATEGORY_B_IRI_HTTPS
	}
}
impl PartialEq<FdAcategoryBIri> for &str {
	fn eq(&self, other: &FdAcategoryBIri) -> bool {
		*self == FD_ACATEGORY_B_IRI_HTTP || *self == FD_ACATEGORY_B_IRI_HTTPS
	}
}
pub struct FdAcategoryBIriOrLabel;
impl PartialEq<&str> for FdAcategoryBIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FdAcategoryBIri || *other == FD_ACATEGORY_B_LABEL
	}
}
impl PartialEq<FdAcategoryBIriOrLabel> for &str {
	fn eq(&self, other: &FdAcategoryBIriOrLabel) -> bool {
		*self == FdAcategoryBIri || *self == FD_ACATEGORY_B_LABEL
	}
}
