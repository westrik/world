//use crate::models::item::{Item, ItemQueryError};
//use crate::db;
//use crate::db::get_conn;
//
pub mod filters;
mod handlers;
//
//#[derive(Serialize)]
//pub struct GetItemResponse {
//    error: Option<String>,
//    items: Option<Vec<Item>>,
//}
//
//fn run_get_items(token: String, pool: &db::PgPool) -> Result<Vec<Item>, ItemQueryError> {
//    Ok(Item::find_all_for_user(&get_conn(&pool).unwrap(), token)?)
//}

// pub async fn get_items(
//     req: HttpRequest,
//     pool: web::Data<db::PgPool>,
// ) -> Result<HttpResponse, Error> {
//     if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
//         let token = String::from(
//             auth_header
//                 .clone()
//                 .to_str()
//                 .map_err(|_| HttpResponse::BadRequest().body("bad token"))?,
//         );
//         let items: Vec<Item> = web::block(move || run_get_items(token, &pool))
//             .await
//             .map_err(|_| HttpResponse::BadRequest().body("failed to find items"))?;
//         Ok(HttpResponse::Ok().json(GetItemResponse {
//             error: None,
//             items: Some(items),
//         }))
//     } else {
//         Ok(HttpResponse::BadRequest().body("no token"))
//     }
// }

//#[derive(Deserialize)]
//pub struct NewItem {
//    content: String,
//}
//
//#[derive(Serialize)]
//pub struct CreateItemResponse {
//    error: Option<String>,
//    item: Option<Item>,
//}
//
//fn run_create_item(
//    token: String,
//    content: String,
//    pool: &db::PgPool,
//) -> Result<Item, ItemQueryError> {
//    Ok(Item::create(&get_conn(&pool).unwrap(), token, content)?)
//}
//
// pub async fn create_item(
//     req: HttpRequest,
//     item: Json<NewItem>,
//     pool: web::Data<db::PgPool>,
// ) -> Result<HttpResponse, Error> {
//     let content = String::from(&item.content);
//     if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
//         let token = String::from(
//             auth_header
//                 .clone()
//                 .to_str()
//                 .map_err(|_| HttpResponse::BadRequest().body("bad token"))?,
//         );
//         let item: Item = web::block(move || run_create_item(token, content, &pool))
//             .await
//             .map_err(|_| HttpResponse::BadRequest().body("failed to create item"))?;
//         Ok(HttpResponse::Ok().json(CreateItemResponse {
//             error: None,
//             item: Some(item),
//         }))
//     } else {
//         Ok(HttpResponse::BadRequest().body("no token"))
//     }
// }
