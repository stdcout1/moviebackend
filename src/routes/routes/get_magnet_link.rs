use crate::database::connect_to_db::MongoDB;
use crate::helper::{parse_id_and_find_user_by_id, FindUserById};
use crate::models::hello_response::HelloNameResponse;
use crate::routes::routes::HelloNameError;
use crate::{ErrorResponse, Status, UNAUTHORIZED};

use crate::routes::authorization::token::request_access_token::AuthorizedUser;
use rocket::serde::json::Json;
use rocket::State;

//(private) request with authorization model (token)
#[get("/private/get_magnet_link")]
pub async fn get_magnet_link(
    auth: AuthorizedUser,
    database: &State<MongoDB>,
) -> Result<Json<&str>, (Status, Json<ErrorResponse>)> {
    
}

//seperate request for 1337x and yts but similar 
