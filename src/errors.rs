use actix_web::http::StatusCode;

pub trait MapErrToInternal<T, E> {
    fn map_err_to_internal(self) -> actix_web::Result<T, actix_web::error::InternalError<E>>;
}
impl<T, E> MapErrToInternal<T, E> for Result<T, E> {
    fn map_err_to_internal(self) -> actix_web::Result<T, actix_web::error::InternalError<E>> {
        self.map_err(|error| {
            actix_web::error::InternalError::new(error, StatusCode::INTERNAL_SERVER_ERROR)
        })
    }
}
