/// <https://schema.org/3DModel>
pub const MODEL_3_D_IRI_HTTP: &str = "http://schema.org/3DModel";
/// <https://schema.org/3DModel>
pub const MODEL_3_D_IRI_HTTPS: &str = "https://schema.org/3DModel";
/// <https://schema.org/3DModel>
pub const MODEL_3_D_LABEL: &str = "Model3D";
pub struct Model3DIri;
impl PartialEq<&str> for Model3DIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MODEL_3_D_IRI_HTTP || *other == MODEL_3_D_IRI_HTTPS
	}
}
impl PartialEq<Model3DIri> for &str {
	fn eq(&self, other: &Model3DIri) -> bool {
		*self == MODEL_3_D_IRI_HTTP || *self == MODEL_3_D_IRI_HTTPS
	}
}
pub struct Model3DIriOrLabel;
impl PartialEq<&str> for Model3DIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == Model3DIri || *other == MODEL_3_D_LABEL
	}
}
impl PartialEq<Model3DIriOrLabel> for &str {
	fn eq(&self, other: &Model3DIriOrLabel) -> bool {
		*self == Model3DIri || *self == MODEL_3_D_LABEL
	}
}
