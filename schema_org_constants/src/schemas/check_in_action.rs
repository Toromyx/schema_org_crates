/// <https://schema.org/CheckInAction>
pub const CHECK_IN_ACTION_IRI_HTTP: &str = "http://schema.org/CheckInAction";
/// <https://schema.org/CheckInAction>
pub const CHECK_IN_ACTION_IRI_HTTPS: &str = "https://schema.org/CheckInAction";
/// <https://schema.org/CheckInAction>
pub const CHECK_IN_ACTION_LABEL: &str = "CheckInAction";
pub struct CheckInActionIri;
impl PartialEq<&str> for CheckInActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHECK_IN_ACTION_IRI_HTTP || *other == CHECK_IN_ACTION_IRI_HTTPS
	}
}
impl PartialEq<CheckInActionIri> for &str {
	fn eq(&self, other: &CheckInActionIri) -> bool {
		*self == CHECK_IN_ACTION_IRI_HTTP || *self == CHECK_IN_ACTION_IRI_HTTPS
	}
}
pub struct CheckInActionIriOrLabel;
impl PartialEq<&str> for CheckInActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CheckInActionIri || *other == CHECK_IN_ACTION_LABEL
	}
}
impl PartialEq<CheckInActionIriOrLabel> for &str {
	fn eq(&self, other: &CheckInActionIriOrLabel) -> bool {
		*self == CheckInActionIri || *self == CHECK_IN_ACTION_LABEL
	}
}
