/// <https://schema.org/UnRegisterAction>
pub const UN_REGISTER_ACTION_IRI_HTTP: &str = "http://schema.org/UnRegisterAction";
/// <https://schema.org/UnRegisterAction>
pub const UN_REGISTER_ACTION_IRI_HTTPS: &str = "https://schema.org/UnRegisterAction";
/// <https://schema.org/UnRegisterAction>
pub const UN_REGISTER_ACTION_LABEL: &str = "UnRegisterAction";
pub struct UnRegisterActionIri;
impl PartialEq<&str> for UnRegisterActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UN_REGISTER_ACTION_IRI_HTTP || *other == UN_REGISTER_ACTION_IRI_HTTPS
	}
}
impl PartialEq<UnRegisterActionIri> for &str {
	fn eq(&self, other: &UnRegisterActionIri) -> bool {
		*self == UN_REGISTER_ACTION_IRI_HTTP || *self == UN_REGISTER_ACTION_IRI_HTTPS
	}
}
pub struct UnRegisterActionIriOrLabel;
impl PartialEq<&str> for UnRegisterActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnRegisterActionIri || *other == UN_REGISTER_ACTION_LABEL
	}
}
impl PartialEq<UnRegisterActionIriOrLabel> for &str {
	fn eq(&self, other: &UnRegisterActionIriOrLabel) -> bool {
		*self == UnRegisterActionIri || *self == UN_REGISTER_ACTION_LABEL
	}
}
