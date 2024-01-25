/// <https://schema.org/NonprofitANBI>
pub const NONPROFIT_ANBI_IRI_HTTP: &str = "http://schema.org/NonprofitANBI";
/// <https://schema.org/NonprofitANBI>
pub const NONPROFIT_ANBI_IRI_HTTPS: &str = "https://schema.org/NonprofitANBI";
/// <https://schema.org/NonprofitANBI>
pub const NONPROFIT_ANBI_LABEL: &str = "NonprofitANBI";
pub struct NonprofitAnbiIri;
impl PartialEq<&str> for NonprofitAnbiIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_ANBI_IRI_HTTP || *other == NONPROFIT_ANBI_IRI_HTTPS
	}
}
impl PartialEq<NonprofitAnbiIri> for &str {
	fn eq(&self, other: &NonprofitAnbiIri) -> bool {
		*self == NONPROFIT_ANBI_IRI_HTTP || *self == NONPROFIT_ANBI_IRI_HTTPS
	}
}
pub struct NonprofitAnbiIriOrLabel;
impl PartialEq<&str> for NonprofitAnbiIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NonprofitAnbiIri || *other == NONPROFIT_ANBI_LABEL
	}
}
impl PartialEq<NonprofitAnbiIriOrLabel> for &str {
	fn eq(&self, other: &NonprofitAnbiIriOrLabel) -> bool {
		*self == NonprofitAnbiIri || *self == NONPROFIT_ANBI_LABEL
	}
}
