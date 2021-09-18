use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use sailfish::TemplateOnce;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let ctx = HelloTemplate {
        messages: vec![String::from("foo"), String::from("bar")],
    };

    let links = vec![
        Link {
            href: "some-address-1",
            text: "some-link-text-1",
        },
        Link {
            href: "some-address-2",
            text: "some-link-text-2",
        },
        Link {
            href: "some-address-3",
            text: "some-link-text-3",
        },
    ];

    let left_sidebar = LeftSidebar { links };

    let txt = TextParagraph{
        text: "some_paragraph"
    };

    Ok(Response::new((left_sidebar.render_once().unwrap()  + &txt.render_once().unwrap() + "Hello, World").into()))
}

#[tokio::main]
async fn main() {

    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}


#[derive(TemplateOnce)]
#[template(path = "hello.html")]
struct HelloTemplate {
    messages: Vec<String>,
}

#[derive(TemplateOnce)]
#[template(path = "components/link.html")]
struct Link<'a, 'b> {
    href: &'a str,
    text: &'b str,
}

#[derive(TemplateOnce)]
#[template(path = "components/left-sidebar.html")]
struct LeftSidebar<'a, 'b> {
    links: Vec<Link<'a, 'b>>,
}

#[derive(TemplateOnce)]
#[template(path = "components/text-paragraph.html")]
struct TextParagraph<'a>{
    text: &'a str
}