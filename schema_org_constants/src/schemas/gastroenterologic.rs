/// <https://schema.org/Gastroenterologic>
pub const GASTROENTEROLOGIC_IRI_HTTP: &str = "http://schema.org/Gastroenterologic";
/// <https://schema.org/Gastroenterologic>
pub const GASTROENTEROLOGIC_IRI_HTTPS: &str = "https://schema.org/Gastroenterologic";
/// <https://schema.org/Gastroenterologic>
pub const GASTROENTEROLOGIC_LABEL: &str = "Gastroenterologic";
pub struct GastroenterologicIri;
impl PartialEq<&str> for GastroenterologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GASTROENTEROLOGIC_IRI_HTTP || *other == GASTROENTEROLOGIC_IRI_HTTPS
	}
}
impl PartialEq<GastroenterologicIri> for &str {
	fn eq(&self, other: &GastroenterologicIri) -> bool {
		*self == GASTROENTEROLOGIC_IRI_HTTP || *self == GASTROENTEROLOGIC_IRI_HTTPS
	}
}
pub struct GastroenterologicIriOrLabel;
impl PartialEq<&str> for GastroenterologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GastroenterologicIri || *other == GASTROENTEROLOGIC_LABEL
	}
}
impl PartialEq<GastroenterologicIriOrLabel> for &str {
	fn eq(&self, other: &GastroenterologicIriOrLabel) -> bool {
		*self == GastroenterologicIri || *self == GASTROENTEROLOGIC_LABEL
	}
}
