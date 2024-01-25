/// <https://schema.org/PoliticalParty>
pub const POLITICAL_PARTY_IRI_HTTP: &str = "http://schema.org/PoliticalParty";
/// <https://schema.org/PoliticalParty>
pub const POLITICAL_PARTY_IRI_HTTPS: &str = "https://schema.org/PoliticalParty";
/// <https://schema.org/PoliticalParty>
pub const POLITICAL_PARTY_LABEL: &str = "PoliticalParty";
pub struct PoliticalPartyIri;
impl PartialEq<&str> for PoliticalPartyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POLITICAL_PARTY_IRI_HTTP || *other == POLITICAL_PARTY_IRI_HTTPS
	}
}
impl PartialEq<PoliticalPartyIri> for &str {
	fn eq(&self, other: &PoliticalPartyIri) -> bool {
		*self == POLITICAL_PARTY_IRI_HTTP || *self == POLITICAL_PARTY_IRI_HTTPS
	}
}
pub struct PoliticalPartyIriOrLabel;
impl PartialEq<&str> for PoliticalPartyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PoliticalPartyIri || *other == POLITICAL_PARTY_LABEL
	}
}
impl PartialEq<PoliticalPartyIriOrLabel> for &str {
	fn eq(&self, other: &PoliticalPartyIriOrLabel) -> bool {
		*self == PoliticalPartyIri || *self == POLITICAL_PARTY_LABEL
	}
}
