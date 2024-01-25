/// <https://schema.org/Homeopathic>
pub const HOMEOPATHIC_IRI_HTTP: &str = "http://schema.org/Homeopathic";
/// <https://schema.org/Homeopathic>
pub const HOMEOPATHIC_IRI_HTTPS: &str = "https://schema.org/Homeopathic";
/// <https://schema.org/Homeopathic>
pub const HOMEOPATHIC_LABEL: &str = "Homeopathic";
pub struct HomeopathicIri;
impl PartialEq<&str> for HomeopathicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HOMEOPATHIC_IRI_HTTP || *other == HOMEOPATHIC_IRI_HTTPS
	}
}
impl PartialEq<HomeopathicIri> for &str {
	fn eq(&self, other: &HomeopathicIri) -> bool {
		*self == HOMEOPATHIC_IRI_HTTP || *self == HOMEOPATHIC_IRI_HTTPS
	}
}
pub struct HomeopathicIriOrLabel;
impl PartialEq<&str> for HomeopathicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HomeopathicIri || *other == HOMEOPATHIC_LABEL
	}
}
impl PartialEq<HomeopathicIriOrLabel> for &str {
	fn eq(&self, other: &HomeopathicIriOrLabel) -> bool {
		*self == HomeopathicIri || *self == HOMEOPATHIC_LABEL
	}
}
