/// <https://schema.org/Fungus>
pub const FUNGUS_IRI_HTTP: &str = "http://schema.org/Fungus";
/// <https://schema.org/Fungus>
pub const FUNGUS_IRI_HTTPS: &str = "https://schema.org/Fungus";
/// <https://schema.org/Fungus>
pub const FUNGUS_LABEL: &str = "Fungus";
pub struct FungusIri;
impl PartialEq<&str> for FungusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == FUNGUS_IRI_HTTP || *other == FUNGUS_IRI_HTTPS
	}
}
impl PartialEq<FungusIri> for &str {
	fn eq(&self, other: &FungusIri) -> bool {
		*self == FUNGUS_IRI_HTTP || *self == FUNGUS_IRI_HTTPS
	}
}
pub struct FungusIriOrLabel;
impl PartialEq<&str> for FungusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == FungusIri || *other == FUNGUS_LABEL
	}
}
impl PartialEq<FungusIriOrLabel> for &str {
	fn eq(&self, other: &FungusIriOrLabel) -> bool {
		*self == FungusIri || *self == FUNGUS_LABEL
	}
}
