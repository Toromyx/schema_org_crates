/// <https://schema.org/Hematologic>
pub const HEMATOLOGIC_IRI_HTTP: &str = "http://schema.org/Hematologic";
/// <https://schema.org/Hematologic>
pub const HEMATOLOGIC_IRI_HTTPS: &str = "https://schema.org/Hematologic";
/// <https://schema.org/Hematologic>
pub const HEMATOLOGIC_LABEL: &str = "Hematologic";
pub struct HematologicIri;
impl PartialEq<&str> for HematologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEMATOLOGIC_IRI_HTTP || *other == HEMATOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<HematologicIri> for &str {
	fn eq(&self, other: &HematologicIri) -> bool {
		*self == HEMATOLOGIC_IRI_HTTP || *self == HEMATOLOGIC_IRI_HTTPS
	}
}
pub struct HematologicIriOrLabel;
impl PartialEq<&str> for HematologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HematologicIri || *other == HEMATOLOGIC_LABEL
	}
}
impl PartialEq<HematologicIriOrLabel> for &str {
	fn eq(&self, other: &HematologicIriOrLabel) -> bool {
		*self == HematologicIri || *self == HEMATOLOGIC_LABEL
	}
}
