use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use common::api::auth::{LoginRequest, LoginResponse};
use common::api::errors::ErrorResponse;
use common::api::nodes::{CreateNodeRequest, MoveNodeRequest, NodeResponse, UpdateNodeRequest};
use common::api::users::{
    ChangePasswordRequest, RegisterRequest, UpdateEmailRequest, UpdateProfileRequest,
    UpdateUsernameRequest, UserResponse,
};
use common::api::workspaces::{CreateWorkspaceRequest, UpdateWorkspaceRequest, WorkspaceResponse};

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
        crate::nodes::routers::create_node,
        crate::nodes::routers::list_nodes,
        crate::nodes::routers::get_node,
        crate::nodes::routers::update_node,
        crate::nodes::routers::delete_node,
        crate::nodes::routers::move_node,
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
        CreateNodeRequest,
        UpdateNodeRequest,
        NodeResponse,
        MoveNodeRequest,
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "workspaces", description = "Workspace management endpoints"),
        (name = "nodes", description = "Tree node management endpoints"),
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
