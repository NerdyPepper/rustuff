use {
    futures::{
        compat::Future01CompatExt,
        future::{FutureExt, TryFutureExt},
    },
    hyper::{rt::run, service::service_fn, Body, Client, Request, Response, Server, Uri},
    std::net::SocketAddr,
};

async fn serve_req(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let url_str = "http://www.rust-lang.org/en-US/";
    let url = url_str.parse::<Uri>().expect("failed to parse URL");
    println!("Got request at {}", req.uri());

    let res = Client::new().get(url).compat().await;
    println!("request finished-- returning response");
    res
}

async fn run_server(addr: SocketAddr) {
    let serve_fut = Server::bind(&addr)
        .serve(|| service_fn(|req| serve_req(req).boxed().compat()));

    if let Err(e) = serve_fut.compat().await {
        eprintln!("server error: {}", e);
    }
}

fn main() {
    let sock_addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    let fut3 = run_server(sock_addr);
    let fut1 = fut3.unit_error().boxed().compat();

    run(fut1);
}
