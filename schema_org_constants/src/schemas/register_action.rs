/// <https://schema.org/RegisterAction>
pub const REGISTER_ACTION_IRI_HTTP: &str = "http://schema.org/RegisterAction";
/// <https://schema.org/RegisterAction>
pub const REGISTER_ACTION_IRI_HTTPS: &str = "https://schema.org/RegisterAction";
/// <https://schema.org/RegisterAction>
pub const REGISTER_ACTION_LABEL: &str = "RegisterAction";
pub struct RegisterActionIri;
impl PartialEq<&str> for RegisterActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REGISTER_ACTION_IRI_HTTP || *other == REGISTER_ACTION_IRI_HTTPS
	}
}
impl PartialEq<RegisterActionIri> for &str {
	fn eq(&self, other: &RegisterActionIri) -> bool {
		*self == REGISTER_ACTION_IRI_HTTP || *self == REGISTER_ACTION_IRI_HTTPS
	}
}
pub struct RegisterActionIriOrLabel;
impl PartialEq<&str> for RegisterActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RegisterActionIri || *other == REGISTER_ACTION_LABEL
	}
}
impl PartialEq<RegisterActionIriOrLabel> for &str {
	fn eq(&self, other: &RegisterActionIriOrLabel) -> bool {
		*self == RegisterActionIri || *self == REGISTER_ACTION_LABEL
	}
}
