/// <https://schema.org/Virus>
pub const VIRUS_IRI_HTTP: &str = "http://schema.org/Virus";
/// <https://schema.org/Virus>
pub const VIRUS_IRI_HTTPS: &str = "https://schema.org/Virus";
/// <https://schema.org/Virus>
pub const VIRUS_LABEL: &str = "Virus";
pub struct VirusIri;
impl PartialEq<&str> for VirusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIRUS_IRI_HTTP || *other == VIRUS_IRI_HTTPS
	}
}
impl PartialEq<VirusIri> for &str {
	fn eq(&self, other: &VirusIri) -> bool {
		*self == VIRUS_IRI_HTTP || *self == VIRUS_IRI_HTTPS
	}
}
pub struct VirusIriOrLabel;
impl PartialEq<&str> for VirusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VirusIri || *other == VIRUS_LABEL
	}
}
impl PartialEq<VirusIriOrLabel> for &str {
	fn eq(&self, other: &VirusIriOrLabel) -> bool {
		*self == VirusIri || *self == VIRUS_LABEL
	}
}
