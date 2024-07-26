use sea_orm_migration::prelude::*;
    pub struct CreateUserTable;

    #[async_trait::async_trait]
    impl MigrationTrait for CreateUserTable {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(User::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(User::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(User::UserName).string().not_null())
                        .col(ColumnDef::new(User::Name).string().null())
                        .col(ColumnDef::new(User::Password).string().null())
                        .col(ColumnDef::new(User::NickName).string().null())
                        .col(ColumnDef::new(User::Avatar).string().null())
                        .col(ColumnDef::new(User::Email).string().null())
                        .col(ColumnDef::new(User::Sex).integer().null())
                        .col(ColumnDef::new(User::PhoneNumber).string().null())
                        .col(ColumnDef::new(User::LastLoginIp).string().null())
                        .col(ColumnDef::new(User::LastLoginTime).string().null())
                        .col(ColumnDef::new(User::Status).integer().not_null())
                        .col(ColumnDef::new(User::CreateTime).timestamp().null())
                        .col(ColumnDef::new(User::UpdateTime).timestamp().null())
                        .col(ColumnDef::new(User::SoflDelete).integer().not_null())
                        .col(ColumnDef::new(User::CreateUserId).integer().null())
                        .col(ColumnDef::new(User::UpdateUserId).integer().null())
                        .col(ColumnDef::new(User::Code).string().null())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager.drop_table(Table::drop().table(User::Table).to_owned()).await
        }
    }

    impl MigrationName for CreateUserTable {
        fn name(&self) -> &str {
            "m20230624010101_create_user_table"
        }
    }
    #[derive(Iden)]
    pub enum User {
        Table,
        Id,
        UserName,
        Name,
        Password,
        NickName,
        Avatar,
        Email,
        Sex,
        PhoneNumber,
        LastLoginIp,
        LastLoginTime,
        Status,
        CreateTime,
        UpdateTime,
        SoflDelete,
        CreateUserId,
        UpdateUserId,
        Code,
    }


    pub struct CreatePermissionTable;

    #[async_trait::async_trait]
    impl MigrationTrait for CreatePermissionTable {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager
                .create_table(
                    Table::create()
                        .table(Permission::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Permission::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Permission::Name).string().not_null())
                        .col(ColumnDef::new(Permission::Description).string().null())
                        .col(ColumnDef::new(Permission::Status).integer().not_null())
                        .col(ColumnDef::new(Permission::Type).integer().not_null())
                        .col(ColumnDef::new(Permission::Icon).string().null())
                        .col(ColumnDef::new(Permission::Sort).integer().null())
                        .col(ColumnDef::new(Permission::ParentId).string().null())
                        .col(ColumnDef::new(Permission::Path).string().null())
                        .col(ColumnDef::new(Permission::CreateTime).timestamp().null())
                        .col(ColumnDef::new(Permission::UpdateTime).timestamp().null())
                        .col(ColumnDef::new(Permission::SoflDelete).integer().not_null())
                        .col(ColumnDef::new(Permission::CreateUserId).integer().null())
                        .col(ColumnDef::new(Permission::UpdateUserId).integer().null())
                        .col(ColumnDef::new(Permission::IsAdminVisible).integer().null())
                        .col(ColumnDef::new(Permission::IsUserVisible).integer().null())
                        .col(ColumnDef::new(Permission::KeyName).string().null())
                        .col(ColumnDef::new(Permission::FileName).string().null())
                        .to_owned(),
                )
                .await
        }

        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager.drop_table(Table::drop().table(Permission::Table).to_owned()).await
        }
    }

    impl MigrationName for CreatePermissionTable {
        fn name(&self) -> &str {
            "m20230624010102_create_permission_table"
        }
    }


    #[derive(Iden)]
    pub enum Permission {
        Table,
        Id,
        Name,
        Description,
        Status,
        Type,
        Icon,
        Sort,
        ParentId,
        Path,
        CreateTime,
        UpdateTime,
        SoflDelete,
        CreateUserId,
        UpdateUserId,
        IsAdminVisible,
        IsUserVisible,
        KeyName,
        FileName,
    }


    pub struct CreateRolePermissionsTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateRolePermissionsTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RolePermissions::RoleId).integer().not_null())
                    .col(ColumnDef::new(RolePermissions::PermissionId).integer().not_null())
                    .col(ColumnDef::new(RolePermissions::CreateTime).timestamp().null())
                    .col(ColumnDef::new(RolePermissions::UpdateTime).timestamp().null())
                    .col(ColumnDef::new(RolePermissions::SoflDelete).integer().not_null())
                    .col(ColumnDef::new(RolePermissions::CreateUserId).integer().null())
                    .col(ColumnDef::new(RolePermissions::UpdateUserId).integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(RolePermissions::Table).to_owned()).await
    }
}

impl MigrationName for CreateRolePermissionsTable {
    fn name(&self) -> &str {
        "m20230624010103_create_role_permissions_table"
    }
}

#[derive(Iden)]
pub enum RolePermissions {
    Table,
    Id,
    RoleId,
    PermissionId,
    CreateTime,
    UpdateTime,
    SoflDelete,
    CreateUserId,
    UpdateUserId,
}

pub struct CreateRoleTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateRoleTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Role::Name).string().not_null())
                    .col(ColumnDef::new(Role::Description).string().null())
                    .col(ColumnDef::new(Role::Status).integer().not_null())
                    .col(ColumnDef::new(Role::Type).integer().not_null())
                    .col(ColumnDef::new(Role::Icon).string().null())
                    .col(ColumnDef::new(Role::Sort).integer().null())
                    .col(ColumnDef::new(Role::CreateTime).timestamp().null())
                    .col(ColumnDef::new(Role::UpdateTime).timestamp().null())
                    .col(ColumnDef::new(Role::SoflDelete).integer().not_null())
                    .col(ColumnDef::new(Role::CreateUserId).integer().null())
                    .col(ColumnDef::new(Role::UpdateUserId).integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Role::Table).to_owned()).await
    }
}

impl MigrationName for CreateRoleTable {
    fn name(&self) -> &str {
        "m20230624010104_create_role_table"
    }
}

#[derive(Iden)]
pub enum Role {
    Table,
    Id,
    Name,
    Description,
    Status,
    Type,
    Icon,
    Sort,
    CreateTime,
    UpdateTime,
    SoflDelete,
    CreateUserId,
    UpdateUserId,
}


pub struct CreateUserRolesTable;

#[async_trait::async_trait]
impl MigrationTrait for CreateUserRolesTable {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserRoles::UserId).integer().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).integer().not_null())
                    .col(ColumnDef::new(UserRoles::Status).integer().not_null())
                    .col(ColumnDef::new(UserRoles::CreateTime).timestamp().null())
                    .col(ColumnDef::new(UserRoles::UpdateTime).timestamp().null())
                    .col(ColumnDef::new(UserRoles::SoflDelete).integer().not_null())
                    .col(ColumnDef::new(UserRoles::CreateUserId).integer().null())
                    .col(ColumnDef::new(UserRoles::UpdateUserId).integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserRoles::Table).to_owned()).await
    }
}

impl MigrationName for CreateUserRolesTable {
    fn name(&self) -> &str {
        "m20230624010105_create_user_roles_table"
    }
}

#[derive(Iden)]
pub enum UserRoles {
    Table,
    Id,
    UserId,
    RoleId,
    Status,
    CreateTime,
    UpdateTime,
    SoflDelete,
    CreateUserId,
    UpdateUserId,
}
