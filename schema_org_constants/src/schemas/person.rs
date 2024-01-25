/// <https://schema.org/Person>
pub const PERSON_IRI_HTTP: &str = "http://schema.org/Person";
/// <https://schema.org/Person>
pub const PERSON_IRI_HTTPS: &str = "https://schema.org/Person";
/// <https://schema.org/Person>
pub const PERSON_LABEL: &str = "Person";
pub struct PersonIri;
impl PartialEq<&str> for PersonIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERSON_IRI_HTTP || *other == PERSON_IRI_HTTPS
	}
}
impl PartialEq<PersonIri> for &str {
	fn eq(&self, other: &PersonIri) -> bool {
		*self == PERSON_IRI_HTTP || *self == PERSON_IRI_HTTPS
	}
}
pub struct PersonIriOrLabel;
impl PartialEq<&str> for PersonIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PersonIri || *other == PERSON_LABEL
	}
}
impl PartialEq<PersonIriOrLabel> for &str {
	fn eq(&self, other: &PersonIriOrLabel) -> bool {
		*self == PersonIri || *self == PERSON_LABEL
	}
}
