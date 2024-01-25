/// <https://schema.org/CheckAction>
pub const CHECK_ACTION_IRI_HTTP: &str = "http://schema.org/CheckAction";
/// <https://schema.org/CheckAction>
pub const CHECK_ACTION_IRI_HTTPS: &str = "https://schema.org/CheckAction";
/// <https://schema.org/CheckAction>
pub const CHECK_ACTION_LABEL: &str = "CheckAction";
pub struct CheckActionIri;
impl PartialEq<&str> for CheckActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECK_ACTION_IRI_HTTP || *other == CHECK_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CheckActionIri> for &str {
	fn eq(&self, other: &CheckActionIri) -> bool {
		*self == CHECK_ACTION_IRI_HTTP || *self == CHECK_ACTION_IRI_HTTPS
	}
}
pub struct CheckActionIriOrLabel;
impl PartialEq<&str> for CheckActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckActionIri || *other == CHECK_ACTION_LABEL
	}
}
impl PartialEq<CheckActionIriOrLabel> for &str {
	fn eq(&self, other: &CheckActionIriOrLabel) -> bool {
		*self == CheckActionIri || *self == CHECK_ACTION_LABEL
	}
}
