mod models;
mod controllers;

use actix_web::{App, HttpServer};
use actix_files as fs;
use tera::Tera;
use crate::controllers::{blog_controller, project_controller};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicializa o Tera para templates
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Erro ao compilar templates: {}", e);
            ::std::process::exit(1);
        }
    };
    
    let tera_arc = actix_web::web::Data::new(tera);

    HttpServer::new(move || {
        App::new()
            .app_data(tera_arc.clone())
            // Servir arquivos estáticos
            .service(fs::Files::new("/static", "static").show_files_listing())
            // Rotas
            .service(blog_controller::index)
            .service(blog_controller::view_post)
            .service(project_controller::index)
            .service(project_controller::view_project)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
