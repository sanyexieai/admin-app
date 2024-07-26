pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::CreateUserTable), 
        Box::new(m20220101_000001_create_table::CreatePermissionTable),
        Box::new(m20220101_000001_create_table::CreateRolePermissionsTable),
        Box::new(m20220101_000001_create_table::CreateRoleTable),
        Box::new(m20220101_000001_create_table::CreateUserRolesTable)]
    }
}
