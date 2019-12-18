use actix_files::NamedFile;
//use actix_session::Session;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, Responder, Result}; // error, web, Error, HttpResponse,
                                         //use futures::Future;

// pub fn index(
//     pool: web::Data<db::PgPool>,
// ) -> impl Future<Item = HttpResponse, Error = Error> {
//     web::block(move || document::get_all(&pool))
//         .from_err()
//         .then(move |res| match res {
//             Ok(documents) => {
//                 let mut context = Context::new();
//                 context.insert("documents", &documents);
//
//                 //Session is set during operations on other endpoints
//                 //that can redirect to index
//                 if let Some(flash) = session::get_flash(&session)? {
//                     context.insert("msg", &(flash.kind, flash.message));
//                     session::clear_flash(&session);
//                 }
//
//                 Ok(HttpResponse::Ok().body(rendered))
//             }
//             Err(e) => Err(e),
//         })
// }

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .respond_to(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}
