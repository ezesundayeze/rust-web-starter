extern crate rocket;
use rocket::{catch, catchers, launch, routes };
mod api;
use api::controllers::user::{create_user, delete_user, get_users, loguser_in, update_user};

#[catch(default)]
fn default_catcher() -> &'static str {
    "An error occurred"
}

#[catch(404)]
fn notfound_catcher() ->  &'static str {
    "Not found"
}

#[catch(401)]
fn unauthorized_catcher() -> &'static str {
    "Unauthorized"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![default_catcher, unauthorized_catcher, notfound_catcher])
        .mount(
            "/user",
            routes![get_users, create_user, update_user, delete_user, loguser_in],
        )
}
