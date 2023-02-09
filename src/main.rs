/* This ia an actix Microservice that has multiple routes:
A. type: "/" that returns a message : "Hello, random best movie around the world!"
B. type: "/movie" that returns a random best movie in the list of the world top 10 best movies
C. type: "/version" that returns the version of the service
*/

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use webdocker::random_movie;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, random best movie around the world!")
}

//create a function that returns a random best movie
#[get("/movie")]
async fn movie() -> impl Responder {
    //print the random movie
    println!("Random Movie: {}", random_movie());
    HttpResponse::Ok().body(random_movie())
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(movie).service(version))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
