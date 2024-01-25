/// <https://schema.org/Hackathon>
pub const HACKATHON_IRI_HTTP: &str = "http://schema.org/Hackathon";
/// <https://schema.org/Hackathon>
pub const HACKATHON_IRI_HTTPS: &str = "https://schema.org/Hackathon";
/// <https://schema.org/Hackathon>
pub const HACKATHON_LABEL: &str = "Hackathon";
pub struct HackathonIri;
impl PartialEq<&str> for HackathonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HACKATHON_IRI_HTTP || *other == HACKATHON_IRI_HTTPS
	}
}
impl PartialEq<HackathonIri> for &str {
	fn eq(&self, other: &HackathonIri) -> bool {
		*self == HACKATHON_IRI_HTTP || *self == HACKATHON_IRI_HTTPS
	}
}
pub struct HackathonIriOrLabel;
impl PartialEq<&str> for HackathonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HackathonIri || *other == HACKATHON_LABEL
	}
}
impl PartialEq<HackathonIriOrLabel> for &str {
	fn eq(&self, other: &HackathonIriOrLabel) -> bool {
		*self == HackathonIri || *self == HACKATHON_LABEL
	}
}
