#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use serde::Serialize;

use rocket::State;
use rocket::fairing::AdHoc;
use rocket::response::NamedFile;
use rocket::response::status::NotFound;
use rocket_contrib::templates::Template;

use diesel::prelude::*;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub mod models;
pub mod schema;
use models::*;

const POSTS_PER_PAGE: usize = 5;

struct StaticURL(String);
#[database("database")]
struct DatabaseConnection(diesel::SqliteConnection);

#[derive(Serialize)]
struct ProjectsContext<'a> {
    pub static_url: &'a str,
    pub projects: Vec<Project>,
}

#[derive(Serialize)]
struct PostsContext<'a> {
    pub static_url: &'a str,
    pub posts: Vec<Post>,
    pub has_next: bool,
    pub next: &'a str,
    pub has_previous: bool,
    pub previous: &'a str,
}

#[derive(Serialize)]
struct PostContext<'a> {
    pub static_url: &'a str,
    pub post: &'a Post,
}

#[get("/")]
fn index(static_url: State<StaticURL>) -> Template {
    let mut context = HashMap::new();
    context.insert("static_url", &static_url.0);

    Template::render("index", &context)
}

#[get("/projects")]
fn projects_page(static_url: State<StaticURL>, database: DatabaseConnection) -> Template {
    use schema::projects::dsl::*;

    let projs = projects
        .order_by(title)
        .load::<models::Project>(&database.0)
        .expect("Error Getting Projects");
    let context = ProjectsContext {
        static_url: &static_url.0,
        projects: projs,
    };

    Template::render("projects", &context)
}

#[get("/blog?<page>")]
fn blog(
    page: Option<usize>,
    static_url: State<StaticURL>,
    database: DatabaseConnection,
) -> Template {
    use schema::posts::dsl::{date, posts};

    let page = match page {
        Some(page) => page,
        None => 0,
    };

    let all_posts = posts
        .order_by(date)
        .load::<models::Post>(&database.0)
        .expect("Error Getting Posts");

    let count = all_posts.len();

    let context = PostsContext {
        static_url: &static_url.0,
        posts: all_posts
            .into_iter()
            .skip(page * POSTS_PER_PAGE)
            .take(POSTS_PER_PAGE)
            .collect(),
        has_next: count > POSTS_PER_PAGE,
        next: &format!("/blog?page={}", page + 1),
        has_previous: page > 0,
        previous: &format!("/blog?page={}", if page == 0 { 0 } else { page - 1 }),
    };

    Template::render("blog", &context)
}

#[get("/blog/<post_id>")]
fn post(post_id: i32, static_url: State<StaticURL>, database: DatabaseConnection) -> Result<Template, NotFound<String>> {
    use schema::posts::dsl::{id, posts};

    let all_posts = posts
        .filter(id.eq(post_id))
        .load::<models::Post>(&database.0)
        .expect("Error Getting Posts");

    if let Some(post) = all_posts.first() {
        let context = PostContext {
            static_url: &static_url.0,
            post,
        };

        Ok(Template::render("post", &context))
    } else {
        Err(NotFound("Invalid Post ID".to_string()))
    }
}

#[get("/about")]
fn about(static_url: State<StaticURL>) -> Template {
    let mut context = HashMap::new();
    context.insert("static_url", &static_url.0);

    Template::render("about", &context)
}

#[get("/static/<asset..>")]
fn static_asset(asset: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(asset)).ok()
}

#[catch(404)]
fn not_found() -> Template {
    let mut context = HashMap::new();
    context.insert("static_url", "https://static.pyxxilated.studio");

    Template::render("404", &context)
}

#[catch(500)]
fn internal_error() -> Template {
    let mut context = HashMap::new();
    context.insert("static_url", "https://static.pyxxilated.studio");

    Template::render("500", &context)
}

#[get("/robots.txt")]
fn robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/robots.txt")).ok()
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![index, about, static_asset, projects_page, blog, post, robots],
        )
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("Assets Config", |rocket| {
            let assets_dir = rocket
                .config()
                .get_str("static_url")
                .unwrap_or("/static")
                .to_string();

            Ok(rocket.manage(StaticURL(assets_dir)))
        }))
        .attach(DatabaseConnection::fairing())
        .register(catchers![not_found, internal_error])
        .launch();
}
