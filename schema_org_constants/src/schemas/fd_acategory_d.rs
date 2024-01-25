/// <https://schema.org/FDAcategoryD>
pub const FD_ACATEGORY_D_IRI_HTTP: &str = "http://schema.org/FDAcategoryD";
/// <https://schema.org/FDAcategoryD>
pub const FD_ACATEGORY_D_IRI_HTTPS: &str = "https://schema.org/FDAcategoryD";
/// <https://schema.org/FDAcategoryD>
pub const FD_ACATEGORY_D_LABEL: &str = "FDAcategoryD";
pub struct FdAcategoryDIri;
impl PartialEq<&str> for FdAcategoryDIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FD_ACATEGORY_D_IRI_HTTP || *other == FD_ACATEGORY_D_IRI_HTTPS
	}
}
impl PartialEq<FdAcategoryDIri> for &str {
	fn eq(&self, other: &FdAcategoryDIri) -> bool {
		*self == FD_ACATEGORY_D_IRI_HTTP || *self == FD_ACATEGORY_D_IRI_HTTPS
	}
}
pub struct FdAcategoryDIriOrLabel;
impl PartialEq<&str> for FdAcategoryDIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FdAcategoryDIri || *other == FD_ACATEGORY_D_LABEL
	}
}
impl PartialEq<FdAcategoryDIriOrLabel> for &str {
	fn eq(&self, other: &FdAcategoryDIriOrLabel) -> bool {
		*self == FdAcategoryDIri || *self == FD_ACATEGORY_D_LABEL
	}
}
