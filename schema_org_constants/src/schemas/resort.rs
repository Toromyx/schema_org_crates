/// <https://schema.org/Resort>
pub const RESORT_IRI_HTTP: &str = "http://schema.org/Resort";
/// <https://schema.org/Resort>
pub const RESORT_IRI_HTTPS: &str = "https://schema.org/Resort";
/// <https://schema.org/Resort>
pub const RESORT_LABEL: &str = "Resort";
pub struct ResortIri;
impl PartialEq<&str> for ResortIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RESORT_IRI_HTTP || *other == RESORT_IRI_HTTPS
	}
}
impl PartialEq<ResortIri> for &str {
	fn eq(&self, other: &ResortIri) -> bool {
		*self == RESORT_IRI_HTTP || *self == RESORT_IRI_HTTPS
	}
}
pub struct ResortIriOrLabel;
impl PartialEq<&str> for ResortIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ResortIri || *other == RESORT_LABEL
	}
}
impl PartialEq<ResortIriOrLabel> for &str {
	fn eq(&self, other: &ResortIriOrLabel) -> bool {
		*self == ResortIri || *self == RESORT_LABEL
	}
}
