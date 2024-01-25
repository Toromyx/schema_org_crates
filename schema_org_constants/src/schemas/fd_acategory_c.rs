/// <https://schema.org/FDAcategoryC>
pub const FD_ACATEGORY_C_IRI_HTTP: &str = "http://schema.org/FDAcategoryC";
/// <https://schema.org/FDAcategoryC>
pub const FD_ACATEGORY_C_IRI_HTTPS: &str = "https://schema.org/FDAcategoryC";
/// <https://schema.org/FDAcategoryC>
pub const FD_ACATEGORY_C_LABEL: &str = "FDAcategoryC";
pub struct FdAcategoryCIri;
impl PartialEq<&str> for FdAcategoryCIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FD_ACATEGORY_C_IRI_HTTP || *other == FD_ACATEGORY_C_IRI_HTTPS
	}
}
impl PartialEq<FdAcategoryCIri> for &str {
	fn eq(&self, other: &FdAcategoryCIri) -> bool {
		*self == FD_ACATEGORY_C_IRI_HTTP || *self == FD_ACATEGORY_C_IRI_HTTPS
	}
}
pub struct FdAcategoryCIriOrLabel;
impl PartialEq<&str> for FdAcategoryCIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FdAcategoryCIri || *other == FD_ACATEGORY_C_LABEL
	}
}
impl PartialEq<FdAcategoryCIriOrLabel> for &str {
	fn eq(&self, other: &FdAcategoryCIriOrLabel) -> bool {
		*self == FdAcategoryCIri || *self == FD_ACATEGORY_C_LABEL
	}
}
