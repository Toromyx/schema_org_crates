/// <https://schema.org/Gynecologic>
pub const GYNECOLOGIC_IRI_HTTP: &str = "http://schema.org/Gynecologic";
/// <https://schema.org/Gynecologic>
pub const GYNECOLOGIC_IRI_HTTPS: &str = "https://schema.org/Gynecologic";
/// <https://schema.org/Gynecologic>
pub const GYNECOLOGIC_LABEL: &str = "Gynecologic";
pub struct GynecologicIri;
impl PartialEq<&str> for GynecologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GYNECOLOGIC_IRI_HTTP || *other == GYNECOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<GynecologicIri> for &str {
	fn eq(&self, other: &GynecologicIri) -> bool {
		*self == GYNECOLOGIC_IRI_HTTP || *self == GYNECOLOGIC_IRI_HTTPS
	}
}
pub struct GynecologicIriOrLabel;
impl PartialEq<&str> for GynecologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GynecologicIri || *other == GYNECOLOGIC_LABEL
	}
}
impl PartialEq<GynecologicIriOrLabel> for &str {
	fn eq(&self, other: &GynecologicIriOrLabel) -> bool {
		*self == GynecologicIri || *self == GYNECOLOGIC_LABEL
	}
}
