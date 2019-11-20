impl_backend_ext! {
    sqlx::MariaDb {
        bool,
        String | &str,
        i16,
        i32,
        i64,
        f32,
        f64
    }
}
