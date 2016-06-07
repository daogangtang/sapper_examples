#![feature(question_mark, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;

extern crate typemap;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate diesel;
extern crate sapper;
extern crate sapper_request_basic_logger;
extern crate sapper_query_params;
#[macro_use] extern crate sapper_body_params;
#[macro_use] extern crate sapper_macros;
extern crate sapper_tmpl;

use std::env;
use std::sync::Arc;
use typemap::Key;
use dotenv::dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use sapper::{SApp, SAppWrapper, Request, Response, Result, SModule};

pub mod schema;
pub mod models;

mod blog;
use blog::BlogModule;


#[derive(Clone)]
struct MyApp;

impl SAppWrapper for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        sapper_request_basic_logger::init(req)?;
        sapper_query_params::process(req)?;
        sapper_body_params::process(req)?;

        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        sapper_request_basic_logger::log(req, res)?;

        Ok(())
    }
}

pub struct AppDB;
impl Key for AppDB { type Value = Arc<PgConnection>; }



pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn main() {
    env_logger::init().unwrap();
    dotenv().ok();
    
    // let conn = Arc::new(establish_connection());
    
    let mut sapp = SApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        // .init_global(Box::new(move |req: &mut Request| -> Result<()> {
        //     req.ext_mut().insert::<AppDB>(conn.clone());
            
        //     Ok(())
        // }))
        .with_wrapper(Box::new(MyApp))
        .add_module(Box::new(BlogModule));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run();
    
}