/// <https://schema.org/Plumber>
pub const PLUMBER_IRI_HTTP: &str = "http://schema.org/Plumber";
/// <https://schema.org/Plumber>
pub const PLUMBER_IRI_HTTPS: &str = "https://schema.org/Plumber";
/// <https://schema.org/Plumber>
pub const PLUMBER_LABEL: &str = "Plumber";
pub struct PlumberIri;
impl PartialEq<&str> for PlumberIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLUMBER_IRI_HTTP || *other == PLUMBER_IRI_HTTPS
	}
}
impl PartialEq<PlumberIri> for &str {
	fn eq(&self, other: &PlumberIri) -> bool {
		*self == PLUMBER_IRI_HTTP || *self == PLUMBER_IRI_HTTPS
	}
}
pub struct PlumberIriOrLabel;
impl PartialEq<&str> for PlumberIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlumberIri || *other == PLUMBER_LABEL
	}
}
impl PartialEq<PlumberIriOrLabel> for &str {
	fn eq(&self, other: &PlumberIriOrLabel) -> bool {
		*self == PlumberIri || *self == PLUMBER_LABEL
	}
}
