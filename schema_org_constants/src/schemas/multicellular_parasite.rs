/// <https://schema.org/MulticellularParasite>
pub const MULTICELLULAR_PARASITE_IRI_HTTP: &str = "http://schema.org/MulticellularParasite";
/// <https://schema.org/MulticellularParasite>
pub const MULTICELLULAR_PARASITE_IRI_HTTPS: &str = "https://schema.org/MulticellularParasite";
/// <https://schema.org/MulticellularParasite>
pub const MULTICELLULAR_PARASITE_LABEL: &str = "MulticellularParasite";
pub struct MulticellularParasiteIri;
impl PartialEq<&str> for MulticellularParasiteIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MULTICELLULAR_PARASITE_IRI_HTTP || *other == MULTICELLULAR_PARASITE_IRI_HTTPS
	}
}
impl PartialEq<MulticellularParasiteIri> for &str {
	fn eq(&self, other: &MulticellularParasiteIri) -> bool {
		*self == MULTICELLULAR_PARASITE_IRI_HTTP || *self == MULTICELLULAR_PARASITE_IRI_HTTPS
	}
}
pub struct MulticellularParasiteIriOrLabel;
impl PartialEq<&str> for MulticellularParasiteIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MulticellularParasiteIri || *other == MULTICELLULAR_PARASITE_LABEL
	}
}
impl PartialEq<MulticellularParasiteIriOrLabel> for &str {
	fn eq(&self, other: &MulticellularParasiteIriOrLabel) -> bool {
		*self == MulticellularParasiteIri || *self == MULTICELLULAR_PARASITE_LABEL
	}
}
