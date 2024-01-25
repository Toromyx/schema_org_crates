/// <https://schema.org/Nonprofit527>
pub const NONPROFIT_527_IRI_HTTP: &str = "http://schema.org/Nonprofit527";
/// <https://schema.org/Nonprofit527>
pub const NONPROFIT_527_IRI_HTTPS: &str = "https://schema.org/Nonprofit527";
/// <https://schema.org/Nonprofit527>
pub const NONPROFIT_527_LABEL: &str = "Nonprofit527";
pub struct Nonprofit527Iri;
impl PartialEq<&str> for Nonprofit527Iri {
	fn eq(&self, other: &&str) -> bool {
		*other == NONPROFIT_527_IRI_HTTP || *other == NONPROFIT_527_IRI_HTTPS
	}
}
impl PartialEq<Nonprofit527Iri> for &str {
	fn eq(&self, other: &Nonprofit527Iri) -> bool {
		*self == NONPROFIT_527_IRI_HTTP || *self == NONPROFIT_527_IRI_HTTPS
	}
}
pub struct Nonprofit527IriOrLabel;
impl PartialEq<&str> for Nonprofit527IriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Nonprofit527Iri || *other == NONPROFIT_527_LABEL
	}
}
impl PartialEq<Nonprofit527IriOrLabel> for &str {
	fn eq(&self, other: &Nonprofit527IriOrLabel) -> bool {
		*self == Nonprofit527Iri || *self == NONPROFIT_527_LABEL
	}
}
