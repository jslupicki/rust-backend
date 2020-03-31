use actix_http::http::Method;

#[derive(Debug)]
pub struct UrlCall {
    pub url: &'static str,
    pub method: Method,
    pub guarded: bool,
}

lazy_static! {
    pub static ref URLS: Vec<UrlCall> = vec![
        UrlCall{
            url: "/auth",
            method: Method::POST,
            guarded: false,
        },
        UrlCall{
            url: "/auth/template",
            method: Method::GET,
            guarded: false,
        },
        UrlCall{
            url: "/users/template",
            method: Method::GET,
            guarded: true,
        },
        UrlCall{
            url: "/users/1",
            method: Method::GET,
            guarded: true,
        },
        UrlCall{
            url: "/users",
            method: Method::GET,
            guarded: true,
        },
        UrlCall{
            url: "/users",
            method: Method::PUT,
            guarded: true,
        },
        UrlCall{
            url: "/users",
            method: Method::POST,
            guarded: true,
        },
        UrlCall{
            url: "/employees/template",
            method: Method::GET,
            guarded: true,
        },
        UrlCall{
            url: "/employees/1",
            method: Method::GET,
            guarded: true,
        },
        UrlCall{
            url: "/employees",
            method: Method::GET,
            guarded: true,
        },
        UrlCall{
            url: "/employees",
            method: Method::PUT,
            guarded: true,
        },
        UrlCall{
            url: "/employees",
            method: Method::POST,
            guarded: true,
        },
        // IMPORTANT: this call have to be last as it logout the session
        UrlCall{
            url: "/auth",
            method: Method::DELETE,
            guarded: false,
        },
    ];
}
