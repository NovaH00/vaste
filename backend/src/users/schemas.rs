use super::models::User;
use common::api::users::UserResponse;

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            display_name: user.display_name,
            bio: user.bio,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}


