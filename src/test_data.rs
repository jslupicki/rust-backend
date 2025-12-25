use actix_web::http::Method;

#[derive(Debug)]
pub struct UrlCall {
    pub url: &'static str,
    pub method: Method,
    pub guarded: bool,
    pub have_to_be_admin: bool,
}

lazy_static! {
    pub static ref URLS: Vec<UrlCall> = vec![
        UrlCall{
            url: "/auth",
            method: Method::POST,
            guarded: false,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/auth/template",
            method: Method::GET,
            guarded: false,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/users/template",
            method: Method::GET,
            guarded: true,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/users/1",
            method: Method::GET,
            guarded: true,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/users",
            method: Method::GET,
            guarded: true,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/users",
            method: Method::PUT,
            guarded: true,
            have_to_be_admin: true,
        },
        UrlCall{
            url: "/users",
            method: Method::POST,
            guarded: true,
            have_to_be_admin: true,
        },
        UrlCall{
            url: "/employees/template",
            method: Method::GET,
            guarded: true,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/employees/1",
            method: Method::GET,
            guarded: true,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/employees/1",
            method: Method::DELETE,
            guarded: true,
            have_to_be_admin: true,
        },
        UrlCall{
            url: "/employees",
            method: Method::GET,
            guarded: true,
            have_to_be_admin: false,
        },
        UrlCall{
            url: "/employees",
            method: Method::PUT,
            guarded: true,
            have_to_be_admin: true,
        },
        UrlCall{
            url: "/employees",
            method: Method::POST,
            guarded: true,
            have_to_be_admin: true,
        },
        // IMPORTANT: this call have to be last as it logout the session
        UrlCall{
            url: "/auth",
            method: Method::DELETE,
            guarded: true,
            have_to_be_admin: false,
        },
    ];
}
