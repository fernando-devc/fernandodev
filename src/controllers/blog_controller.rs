use actix_web::{get, web, HttpResponse, Result};
use tera::Tera;
use crate::models::post::Post;
use crate::models::project::Project;
use pulldown_cmark::{Parser, html::push_html};
use std::fs;
use crate::controllers::project_controller::list_projects;

fn read_post(slug: &str) -> Option<Post> {
    let path = format!("posts/{}.md", slug);
    println!("Tentando ler o arquivo: {}", path); // Log para debug
    
    if let Ok(content) = fs::read_to_string(&path) {
        if let Some(post) = parse_markdown_post(&content) {
            return Some(post);
        }
    }
    None
}

fn parse_markdown_post(content: &str) -> Option<Post> {
    let parts: Vec<&str> = content.split("---\n").collect();
    if parts.len() >= 3 {
        let metadata = parts[1];
        let content = parts[2];
        

        let mut title = String::new();
        let mut created_at = chrono::Utc::now();
        let mut updated_at = chrono::Utc::now();
        
        for line in metadata.lines() {
            if let Some((key, value)) = line.split_once(':') {
                match key.trim() {
                    "title" => title = value.trim().to_string(),
                    "created_at" => {
                        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(value.trim()) {
                            created_at = date.with_timezone(&chrono::Utc);
                        }
                    },
                    "updated_at" => {
                        if let Ok(date) = chrono::DateTime::parse_from_rfc3339(value.trim()) {
                            updated_at = date.with_timezone(&chrono::Utc);
                        }
                    },
                    _ => {}
                }
            }
        }
        
        let mut html_output = String::new();
        let parser = Parser::new(content);
        push_html(&mut html_output, parser);
        
        let slug = title.to_lowercase().replace(" ", "-");
        
        Some(Post {
            title,
            slug,
            content: html_output,
            created_at,
            updated_at,
        })
    } else {
        None
    }
}

fn list_posts() -> Vec<Post> {
    let mut posts = Vec::new();
    if let Ok(entries) = fs::read_dir("posts") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".md") {
                        let slug = filename.trim_end_matches(".md");
                        if let Some(post) = read_post(slug) {
                            posts.push(post);
                        }
                    }
                }
            }
        }
    }
    posts.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    posts
}

#[get("/")]
pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();
    let posts = list_posts();
    let projects = list_projects();
    
    ctx.insert("posts", &posts);
    ctx.insert("projects", &projects);
    
    let rendered = tmpl.render("index.html", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/post/{slug}")]
pub async fn view_post(
    tmpl: web::Data<Tera>,
    path: web::Path<String>
) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();
    let slug = path.into_inner();
    
    println!("Requisição para post com slug: {}", slug); // Log para debug
    
    if let Some(post) = read_post(&slug) {
        ctx.insert("post", &post);
        let rendered = tmpl.render("post.html", &ctx)
            .map_err(|e| {
                println!("Erro ao renderizar template: {}", e); // Log para debug
                actix_web::error::ErrorInternalServerError("Template error")
            })?;
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        Ok(HttpResponse::NotFound().body(format!("Post não encontrado: {}", slug)))
    }
} 