/// <https://schema.org/NGO>
pub const NGO_IRI_HTTP: &str = "http://schema.org/NGO";
/// <https://schema.org/NGO>
pub const NGO_IRI_HTTPS: &str = "https://schema.org/NGO";
/// <https://schema.org/NGO>
pub const NGO_LABEL: &str = "NGO";
pub struct NgoIri;
impl PartialEq<&str> for NgoIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NGO_IRI_HTTP || *other == NGO_IRI_HTTPS
	}
}
impl PartialEq<NgoIri> for &str {
	fn eq(&self, other: &NgoIri) -> bool {
		*self == NGO_IRI_HTTP || *self == NGO_IRI_HTTPS
	}
}
pub struct NgoIriOrLabel;
impl PartialEq<&str> for NgoIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NgoIri || *other == NGO_LABEL
	}
}
impl PartialEq<NgoIriOrLabel> for &str {
	fn eq(&self, other: &NgoIriOrLabel) -> bool {
		*self == NgoIri || *self == NGO_LABEL
	}
}
