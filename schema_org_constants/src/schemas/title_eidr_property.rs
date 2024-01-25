/// <https://schema.org/titleEIDR>
pub const TITLE_EIDR_PROPERTY_IRI_HTTP: &str = "http://schema.org/titleEIDR";
/// <https://schema.org/titleEIDR>
pub const TITLE_EIDR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/titleEIDR";
/// <https://schema.org/titleEIDR>
pub const TITLE_EIDR_PROPERTY_LABEL: &str = "titleEIDR";
pub struct TitleEidrPropertyIri;
impl PartialEq<&str> for TitleEidrPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TITLE_EIDR_PROPERTY_IRI_HTTP || *other == TITLE_EIDR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TitleEidrPropertyIri> for &str {
	fn eq(&self, other: &TitleEidrPropertyIri) -> bool {
		*self == TITLE_EIDR_PROPERTY_IRI_HTTP || *self == TITLE_EIDR_PROPERTY_IRI_HTTPS
	}
}
pub struct TitleEidrPropertyIriOrLabel;
impl PartialEq<&str> for TitleEidrPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TitleEidrPropertyIri || *other == TITLE_EIDR_PROPERTY_LABEL
	}
}
impl PartialEq<TitleEidrPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TitleEidrPropertyIriOrLabel) -> bool {
		*self == TitleEidrPropertyIri || *self == TITLE_EIDR_PROPERTY_LABEL
	}
}
