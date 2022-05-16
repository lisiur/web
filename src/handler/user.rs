use crate::domain::{UserDb, UserInfoService};
use crate::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    id: Uuid,
    name: String,
    email: Option<String>,
    phone: Option<String>,
}

#[get("/user/detail")]
pub async fn detail(
    user_repo: Db<UserDb>,
    login_user: LoginUser,
) -> Result<JsonResponse<UserDetail>> {
    let user = UserInfoService::new(&*user_repo)
        .exec(&login_user.id)
        .await?;
    Response::json_ok(UserDetail {
        id: user.id,
        name: user.name,
        email: user.email,
        phone: user.phone,
    })
}
