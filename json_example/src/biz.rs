use sapper::Result;
use sapper::Module;
use sapper::Request;
use sapper::Response;
use sapper::Router;

use serde_json;
use std::str;

use sapper_std::{PathParams, FormParams, QueryParams, JsonParams};


#[derive(Clone)]
pub struct Biz;

impl Biz {
    // those handlers in module Biz
    fn index(req: &mut Request) -> Result<Response> {
	let params = get_query_params!(req);
	let foo = t_param!(params, "foo");
	let bar = t_param!(params, "bar");
	let num = t_param_parse!(params, "num", i32);

	println!("{}, {}, {}", foo, bar, num);

	let json2ret = json!({
	    "foo": foo,
	    "bar": bar,
	    "num": num
	});

	res_json!(json2ret)
    }

    fn get_user(req: &mut Request) -> Result<Response> {
	let params = get_path_params!(req);
	let id = t_param!(params, "id").clone();

	println!("{}", id);

	let json2ret = json!({
	    "id": id
	});

	res_json!(json2ret)
    }

    fn get_user_age(req: &mut Request) -> Result<Response> {
	let params = get_path_params!(req);
	let id = t_param!(params, "id").clone();
	let age = t_param_parse!(params, "age", i32);

	println!("{}, {}", id, age);

	let json2ret = json!({
	    "id": id,
	    "age": age
	});

	res_json!(json2ret)
    }

    fn test(req: &mut Request) -> Result<Response> {
	res_json_ok!("success.")
    }

    fn test_post(req: &mut Request) -> Result<Response> {
	let params = get_form_params!(req);
	let foo = t_param!(params, "foo");
	let bar = t_param!(params, "bar");
	let num = t_param_parse!(params, "num", i32);

	println!("{}, {}, {}", foo, bar, num);

	res_json_error!("fail.")
    }

    fn post_json(req: &mut Request) -> Result<Response> {
	#[derive(Serialize, Deserialize, Debug)]
	struct Person {
	    foo: String,
	    bar: String,
	    num: i32,
	}

	let person: Person = get_json_params!(req);

	println!("{:#?}", person);

	res_json_ok!("ok.")
    }

}

// set before, after middleware, and add routers
impl Module for Biz {

    fn before(&self, req: &mut Request) -> Result<()> {
	println!("{}", "in Biz before.");
	Ok(())
    }

    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
	println!("{}", "in Biz after.");

	Ok(())
    }

    // here add routers ....
    fn router(&self, router: &mut Router) -> Result<()> {

	// /?foo=hello&bar=mike&num=100
	router.get("/", Self::index);
	// /index?foo=hello&bar=mike&num=100
	router.get("/index", Self::index);
	// /user/42
	router.get("/user/:id", Self::get_user);
	// /user/42/home
	router.get("/user/:id/home", Self::get_user);
	// /user/42/28
	router.get("/user/:id/:age", Self::get_user_age);

	// /test
	router.post("/test", Self::test);
	// /test_post     foo=hello&bar=mike&num=100
	router.post("/test_post", Self::test_post);
	// /post_json     {"foo": "hello", "bar": "mike", "num": 100}
	router.post("/post_json", Self::post_json);

	Ok(())

    }


}
