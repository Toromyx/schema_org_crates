/// <https://schema.org/FDAcategoryX>
pub const FD_ACATEGORY_X_IRI_HTTP: &str = "http://schema.org/FDAcategoryX";
/// <https://schema.org/FDAcategoryX>
pub const FD_ACATEGORY_X_IRI_HTTPS: &str = "https://schema.org/FDAcategoryX";
/// <https://schema.org/FDAcategoryX>
pub const FD_ACATEGORY_X_LABEL: &str = "FDAcategoryX";
pub struct FdAcategoryXIri;
impl PartialEq<&str> for FdAcategoryXIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FD_ACATEGORY_X_IRI_HTTP || *other == FD_ACATEGORY_X_IRI_HTTPS
	}
}
impl PartialEq<FdAcategoryXIri> for &str {
	fn eq(&self, other: &FdAcategoryXIri) -> bool {
		*self == FD_ACATEGORY_X_IRI_HTTP || *self == FD_ACATEGORY_X_IRI_HTTPS
	}
}
pub struct FdAcategoryXIriOrLabel;
impl PartialEq<&str> for FdAcategoryXIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FdAcategoryXIri || *other == FD_ACATEGORY_X_LABEL
	}
}
impl PartialEq<FdAcategoryXIriOrLabel> for &str {
	fn eq(&self, other: &FdAcategoryXIriOrLabel) -> bool {
		*self == FdAcategoryXIri || *self == FD_ACATEGORY_X_LABEL
	}
}
