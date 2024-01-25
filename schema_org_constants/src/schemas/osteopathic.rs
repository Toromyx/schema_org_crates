/// <https://schema.org/Osteopathic>
pub const OSTEOPATHIC_IRI_HTTP: &str = "http://schema.org/Osteopathic";
/// <https://schema.org/Osteopathic>
pub const OSTEOPATHIC_IRI_HTTPS: &str = "https://schema.org/Osteopathic";
/// <https://schema.org/Osteopathic>
pub const OSTEOPATHIC_LABEL: &str = "Osteopathic";
pub struct OsteopathicIri;
impl PartialEq<&str> for OsteopathicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OSTEOPATHIC_IRI_HTTP || *other == OSTEOPATHIC_IRI_HTTPS
	}
}
impl PartialEq<OsteopathicIri> for &str {
	fn eq(&self, other: &OsteopathicIri) -> bool {
		*self == OSTEOPATHIC_IRI_HTTP || *self == OSTEOPATHIC_IRI_HTTPS
	}
}
pub struct OsteopathicIriOrLabel;
impl PartialEq<&str> for OsteopathicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OsteopathicIri || *other == OSTEOPATHIC_LABEL
	}
}
impl PartialEq<OsteopathicIriOrLabel> for &str {
	fn eq(&self, other: &OsteopathicIriOrLabel) -> bool {
		*self == OsteopathicIri || *self == OSTEOPATHIC_LABEL
	}
}
