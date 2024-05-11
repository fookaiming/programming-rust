use actix_web::{App, HttpResponse, HttpServer, web};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("server starting on http://localhost:3000 ...");
    server.bind("127.0.0.1:3000").expect("error when starting server").run().expect("error when running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html")
        .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
        )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing GCD with zero is boring");
    }

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("The greatest common divisor of the number: {} and {} is <b>{}<b>", form.n, form.m, gcd(form.n, form.m)))
}

todo!(use gcd as lib to replace the fowllowing gcd function);
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}