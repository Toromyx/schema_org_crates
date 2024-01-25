/// <https://schema.org/DefenceEstablishment>
pub const DEFENCE_ESTABLISHMENT_IRI_HTTP: &str = "http://schema.org/DefenceEstablishment";
/// <https://schema.org/DefenceEstablishment>
pub const DEFENCE_ESTABLISHMENT_IRI_HTTPS: &str = "https://schema.org/DefenceEstablishment";
/// <https://schema.org/DefenceEstablishment>
pub const DEFENCE_ESTABLISHMENT_LABEL: &str = "DefenceEstablishment";
pub struct DefenceEstablishmentIri;
impl PartialEq<&str> for DefenceEstablishmentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEFENCE_ESTABLISHMENT_IRI_HTTP || *other == DEFENCE_ESTABLISHMENT_IRI_HTTPS
	}
}
impl PartialEq<DefenceEstablishmentIri> for &str {
	fn eq(&self, other: &DefenceEstablishmentIri) -> bool {
		*self == DEFENCE_ESTABLISHMENT_IRI_HTTP || *self == DEFENCE_ESTABLISHMENT_IRI_HTTPS
	}
}
pub struct DefenceEstablishmentIriOrLabel;
impl PartialEq<&str> for DefenceEstablishmentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DefenceEstablishmentIri || *other == DEFENCE_ESTABLISHMENT_LABEL
	}
}
impl PartialEq<DefenceEstablishmentIriOrLabel> for &str {
	fn eq(&self, other: &DefenceEstablishmentIriOrLabel) -> bool {
		*self == DefenceEstablishmentIri || *self == DEFENCE_ESTABLISHMENT_LABEL
	}
}
