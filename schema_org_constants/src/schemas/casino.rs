/// <https://schema.org/Casino>
pub const CASINO_IRI_HTTP: &str = "http://schema.org/Casino";
/// <https://schema.org/Casino>
pub const CASINO_IRI_HTTPS: &str = "https://schema.org/Casino";
/// <https://schema.org/Casino>
pub const CASINO_LABEL: &str = "Casino";
pub struct CasinoIri;
impl PartialEq<&str> for CasinoIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CASINO_IRI_HTTP || *other == CASINO_IRI_HTTPS
	}
}
impl PartialEq<CasinoIri> for &str {
	fn eq(&self, other: &CasinoIri) -> bool {
		*self == CASINO_IRI_HTTP || *self == CASINO_IRI_HTTPS
	}
}
pub struct CasinoIriOrLabel;
impl PartialEq<&str> for CasinoIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CasinoIri || *other == CASINO_LABEL
	}
}
impl PartialEq<CasinoIriOrLabel> for &str {
	fn eq(&self, other: &CasinoIriOrLabel) -> bool {
		*self == CasinoIri || *self == CASINO_LABEL
	}
}
