/// The method of the HTTP request to send to a server.
#[derive(Clone, PartialEq, Debug)]
pub enum Method {
    Post,
    Get,
    Patch,
    Put,
    Delete,
    Head,
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Post => "POST",
            Method::Get => "GET",
            Method::Patch => "PATCH",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
        }
        .to_string()
    }
}
