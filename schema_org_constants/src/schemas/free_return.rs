/// <https://schema.org/FreeReturn>
pub const FREE_RETURN_IRI_HTTP: &str = "http://schema.org/FreeReturn";
/// <https://schema.org/FreeReturn>
pub const FREE_RETURN_IRI_HTTPS: &str = "https://schema.org/FreeReturn";
/// <https://schema.org/FreeReturn>
pub const FREE_RETURN_LABEL: &str = "FreeReturn";
pub struct FreeReturnIri;
impl PartialEq<&str> for FreeReturnIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FREE_RETURN_IRI_HTTP || *other == FREE_RETURN_IRI_HTTPS
	}
}
impl PartialEq<FreeReturnIri> for &str {
	fn eq(&self, other: &FreeReturnIri) -> bool {
		*self == FREE_RETURN_IRI_HTTP || *self == FREE_RETURN_IRI_HTTPS
	}
}
pub struct FreeReturnIriOrLabel;
impl PartialEq<&str> for FreeReturnIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FreeReturnIri || *other == FREE_RETURN_LABEL
	}
}
impl PartialEq<FreeReturnIriOrLabel> for &str {
	fn eq(&self, other: &FreeReturnIriOrLabel) -> bool {
		*self == FreeReturnIri || *self == FREE_RETURN_LABEL
	}
}
