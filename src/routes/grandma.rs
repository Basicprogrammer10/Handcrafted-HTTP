use afire::{Method, Response, Server, SetCookie};

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/store", |_| {
        Response::new()
            .cookie(SetCookie::new("yummy-cookie", "for-grandma"))
            .text("Please POST this cookie to '/grandma'")
    });

    server.route(Method::POST, "/grandma", |req| {
        let cookie = match req.cookies.iter().find(|x| x.name.to_lowercase() == "yummy-cookie") {
            Some(i) => i,
            None => return Response::new().status(400).text("Where is my cookie!?"),
        };

        if cookie.value != "for-grandma" {
            return Response::new().status(400).text("Thats not my cookie!");
        }

        match req.header("password") {
            Some(i) => {
                if i != "password123" {
                    return Response::new().status(400).text("Incorrect Password!");
                }

                return Response::new()
                    .status(418)
                    .text("Grandma is happy\n\nCongrats on making it through! Hopefully you now know how to make HTTP requests, and yes this information is totally useful in your every day life :tea:")
            },
            None => return Response::new()
                .status(402)
                .text("Unfortunately you haven't supplied us with your account password! please GET it from '/bank' and give it to us along with the original cookie to continue!"),
        };
    });

    server.route(Method::GET, "/bank", |_| {
        Response::new()
            .text("password123\nPlease put this along with the cookie, in the header 'Password' with yout POST to /grandma")
    });
}
