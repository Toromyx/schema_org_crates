/// <https://schema.org/False>
pub const FALSE_IRI_HTTP: &str = "http://schema.org/False";
/// <https://schema.org/False>
pub const FALSE_IRI_HTTPS: &str = "https://schema.org/False";
/// <https://schema.org/False>
pub const FALSE_LABEL: &str = "False";
pub struct FalseIri;
impl PartialEq<&str> for FalseIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FALSE_IRI_HTTP || *other == FALSE_IRI_HTTPS
	}
}
impl PartialEq<FalseIri> for &str {
	fn eq(&self, other: &FalseIri) -> bool {
		*self == FALSE_IRI_HTTP || *self == FALSE_IRI_HTTPS
	}
}
pub struct FalseIriOrLabel;
impl PartialEq<&str> for FalseIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FalseIri || *other == FALSE_LABEL
	}
}
impl PartialEq<FalseIriOrLabel> for &str {
	fn eq(&self, other: &FalseIriOrLabel) -> bool {
		*self == FalseIri || *self == FALSE_LABEL
	}
}
