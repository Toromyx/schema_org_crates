/// <https://schema.org/Psychiatric>
pub const PSYCHIATRIC_IRI_HTTP: &str = "http://schema.org/Psychiatric";
/// <https://schema.org/Psychiatric>
pub const PSYCHIATRIC_IRI_HTTPS: &str = "https://schema.org/Psychiatric";
/// <https://schema.org/Psychiatric>
pub const PSYCHIATRIC_LABEL: &str = "Psychiatric";
pub struct PsychiatricIri;
impl PartialEq<&str> for PsychiatricIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PSYCHIATRIC_IRI_HTTP || *other == PSYCHIATRIC_IRI_HTTPS
	}
}
impl PartialEq<PsychiatricIri> for &str {
	fn eq(&self, other: &PsychiatricIri) -> bool {
		*self == PSYCHIATRIC_IRI_HTTP || *self == PSYCHIATRIC_IRI_HTTPS
	}
}
pub struct PsychiatricIriOrLabel;
impl PartialEq<&str> for PsychiatricIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PsychiatricIri || *other == PSYCHIATRIC_LABEL
	}
}
impl PartialEq<PsychiatricIriOrLabel> for &str {
	fn eq(&self, other: &PsychiatricIriOrLabel) -> bool {
		*self == PsychiatricIri || *self == PSYCHIATRIC_LABEL
	}
}
