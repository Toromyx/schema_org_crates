use std::path::PathBuf;

use strum::EnumIter;

use crate::schema_type::SchemaType;

#[derive(Debug, EnumIter)]
pub enum SchemaOrgCrate {
	Constants,
	Traits,
}

impl SchemaOrgCrate {
	pub fn schemas_dir(&self) -> PathBuf {
		PathBuf::from(match self {
			SchemaOrgCrate::Constants => "../schema_org_constants/src/schemas",
			SchemaOrgCrate::Traits => "../schema_org_traits/src/schemas",
		})
	}

	pub fn schemas_file(&self) -> PathBuf {
		PathBuf::from(match self {
			SchemaOrgCrate::Constants => "../schema_org_constants/src/schemas.rs",
			SchemaOrgCrate::Traits => "../schema_org_traits/src/schemas.rs",
		})
	}

	pub fn schema_types(&self) -> &'static [SchemaType] {
		match self {
			SchemaOrgCrate::Constants => &[],
			SchemaOrgCrate::Traits => &[SchemaType::Class, SchemaType::Property],
		}
	}

	pub fn schema_type_dir(&self, schema_type: &SchemaType) -> PathBuf {
		let schemas_dir = self.schemas_dir();
		match self {
			SchemaOrgCrate::Constants => schemas_dir,
			SchemaOrgCrate::Traits => {
				let mut schema_type_dir = PathBuf::from(&schemas_dir);
				schema_type_dir.push(schema_type.parent_module_name());
				schema_type_dir
			}
		}
	}

	pub fn schema_type_file(&self, schema_type: &SchemaType) -> PathBuf {
		let schemas_dir = self.schemas_dir();
		match self {
			SchemaOrgCrate::Constants => unimplemented!(),
			SchemaOrgCrate::Traits => {
				let mut schema_type_file = PathBuf::from(&schemas_dir);
				schema_type_file.push(format!("{}.rs", schema_type.parent_module_name()));
				schema_type_file
			}
		}
	}
}
