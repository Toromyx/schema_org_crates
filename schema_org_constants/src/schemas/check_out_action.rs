/// <https://schema.org/CheckOutAction>
pub const CHECK_OUT_ACTION_IRI_HTTP: &str = "http://schema.org/CheckOutAction";
/// <https://schema.org/CheckOutAction>
pub const CHECK_OUT_ACTION_IRI_HTTPS: &str = "https://schema.org/CheckOutAction";
/// <https://schema.org/CheckOutAction>
pub const CHECK_OUT_ACTION_LABEL: &str = "CheckOutAction";
pub struct CheckOutActionIri;
impl PartialEq<&str> for CheckOutActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECK_OUT_ACTION_IRI_HTTP || *other == CHECK_OUT_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CheckOutActionIri> for &str {
	fn eq(&self, other: &CheckOutActionIri) -> bool {
		*self == CHECK_OUT_ACTION_IRI_HTTP || *self == CHECK_OUT_ACTION_IRI_HTTPS
	}
}
pub struct CheckOutActionIriOrLabel;
impl PartialEq<&str> for CheckOutActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckOutActionIri || *other == CHECK_OUT_ACTION_LABEL
	}
}
impl PartialEq<CheckOutActionIriOrLabel> for &str {
	fn eq(&self, other: &CheckOutActionIriOrLabel) -> bool {
		*self == CheckOutActionIri || *self == CHECK_OUT_ACTION_LABEL
	}
}
