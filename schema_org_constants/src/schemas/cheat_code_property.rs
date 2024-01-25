/// <https://schema.org/cheatCode>
pub const CHEAT_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/cheatCode";
/// <https://schema.org/cheatCode>
pub const CHEAT_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cheatCode";
/// <https://schema.org/cheatCode>
pub const CHEAT_CODE_PROPERTY_LABEL: &str = "cheatCode";
pub struct CheatCodePropertyIri;
impl PartialEq<&str> for CheatCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHEAT_CODE_PROPERTY_IRI_HTTP || *other == CHEAT_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CheatCodePropertyIri> for &str {
	fn eq(&self, other: &CheatCodePropertyIri) -> bool {
		*self == CHEAT_CODE_PROPERTY_IRI_HTTP || *self == CHEAT_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct CheatCodePropertyIriOrLabel;
impl PartialEq<&str> for CheatCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheatCodePropertyIri || *other == CHEAT_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<CheatCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CheatCodePropertyIriOrLabel) -> bool {
		*self == CheatCodePropertyIri || *self == CHEAT_CODE_PROPERTY_LABEL
	}
}
