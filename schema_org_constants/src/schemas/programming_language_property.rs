/// <https://schema.org/programmingLanguage>
pub const PROGRAMMING_LANGUAGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/programmingLanguage";
/// <https://schema.org/programmingLanguage>
pub const PROGRAMMING_LANGUAGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/programmingLanguage";
/// <https://schema.org/programmingLanguage>
pub const PROGRAMMING_LANGUAGE_PROPERTY_LABEL: &str = "programmingLanguage";
pub struct ProgrammingLanguagePropertyIri;
impl PartialEq<&str> for ProgrammingLanguagePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PROGRAMMING_LANGUAGE_PROPERTY_IRI_HTTP
			|| *other == PROGRAMMING_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ProgrammingLanguagePropertyIri> for &str {
	fn eq(&self, other: &ProgrammingLanguagePropertyIri) -> bool {
		*self == PROGRAMMING_LANGUAGE_PROPERTY_IRI_HTTP
			|| *self == PROGRAMMING_LANGUAGE_PROPERTY_IRI_HTTPS
	}
}
pub struct ProgrammingLanguagePropertyIriOrLabel;
impl PartialEq<&str> for ProgrammingLanguagePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ProgrammingLanguagePropertyIri || *other == PROGRAMMING_LANGUAGE_PROPERTY_LABEL
	}
}
impl PartialEq<ProgrammingLanguagePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ProgrammingLanguagePropertyIriOrLabel) -> bool {
		*self == ProgrammingLanguagePropertyIri || *self == PROGRAMMING_LANGUAGE_PROPERTY_LABEL
	}
}
