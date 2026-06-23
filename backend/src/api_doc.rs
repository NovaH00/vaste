use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use common::api::auth::{LoginRequest, LoginResponse};
use common::api::error::ErrorResponse;
use common::api::users::{
    ChangePasswordRequest, RegisterRequest, UpdateEmailRequest,
    UpdateProfileRequest, UpdateUsernameRequest, UserResponse,
};
use common::api::workspaces::{
    CreateWorkspaceRequest, UpdateWorkspaceRequest, WorkspaceResponse,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::routers::login,
        crate::auth::routers::logout,
        crate::auth::routers::refresh,
        crate::users::routers::register,
        crate::users::routers::get_user,
        crate::users::routers::get_me,
        crate::users::routers::update_username,
        crate::users::routers::update_email,
        crate::users::routers::update_profile,
        crate::users::routers::change_password,
        crate::users::routers::delete_user_handler,
        crate::workspaces::routers::create_workspace,
        crate::workspaces::routers::list_workspaces,
        crate::workspaces::routers::get_workspace,
        crate::workspaces::routers::update_workspace,
        crate::workspaces::routers::delete_workspace,
    ),
    components(schemas(
        LoginRequest,
        LoginResponse,
        ErrorResponse,
        RegisterRequest,
        UserResponse,
        UpdateUsernameRequest,
        UpdateEmailRequest,
        UpdateProfileRequest,
        ChangePasswordRequest,
        CreateWorkspaceRequest,
        UpdateWorkspaceRequest,
        WorkspaceResponse,
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "workspaces", description = "Workspace management endpoints"),
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("UUID")
                        .build(),
                ),
            )
        }
    }
}
