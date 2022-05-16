use crate::prelude::*;
use crate::domain::UserDb;

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
    pool: Data<DbPool>,
    login_user: LoginUser
    ) -> Result<JsonResponse<UserDetail>> {
    todo!()
}
