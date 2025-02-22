use actix_web::{get, web, HttpResponse, Result};
use tera::Tera;
use crate::models::project::Project;
use std::fs;
use pulldown_cmark::{Parser, html::push_html};

fn read_project(slug: &str) -> Option<Project> {
    let path = format!("projects/{}.md", slug);
    println!("Tentando ler o projeto: {}", path);
    
    if let Ok(content) = fs::read_to_string(&path) {
        if let Some(project) = parse_markdown_project(&content) {
            return Some(project);
        }
    }
    None
}

fn parse_markdown_project(content: &str) -> Option<Project> {
    let parts: Vec<&str> = content.split("---\n").collect();
    if parts.len() >= 3 {
        let metadata = parts[1];
        let content = parts[2];
        
        let mut title = String::new();
        let mut description = String::new();
        let mut technologies = Vec::new();
        let mut github_url = None;
        let mut live_url = None;
        let mut created_at = chrono::Utc::now();
        let mut updated_at = chrono::Utc::now();
        
        for line in metadata.lines() {
            if let Some((key, value)) = line.split_once(':') {
                match key.trim() {
                    "title" => title = value.trim().to_string(),
                    "description" => description = value.trim().to_string(),
                    "technologies" => {
                        technologies = value.trim()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    },
                    "github_url" => github_url = Some(value.trim().to_string()),
                    "live_url" => live_url = Some(value.trim().to_string()),
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
        
        Some(Project {
            title,
            slug,
            description,
            content: html_output,
            technologies,
            github_url,
            live_url,
            created_at,
            updated_at,
        })
    } else {
        None
    }
}

pub fn list_projects() -> Vec<Project> {
    let mut projects = Vec::new();
    if let Ok(entries) = fs::read_dir("projects") {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".md") {
                        let slug = filename.trim_end_matches(".md");
                        if let Some(project) = read_project(slug) {
                            projects.push(project);
                        }
                    }
                }
            }
        }
    }
    projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    projects
}

#[get("/projects")]
pub async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();
    let projects = list_projects();
    ctx.insert("projects", &projects);
    
    let rendered = tmpl.render("projects/index.html", &ctx)
        .map_err(|e| {
            println!("Erro ao renderizar template: {}", e);
            actix_web::error::ErrorInternalServerError("Template error")
        })?;
    
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/projects/{slug}")]
pub async fn view_project(
    tmpl: web::Data<Tera>,
    path: web::Path<String>
) -> Result<HttpResponse> {
    let mut ctx = tera::Context::new();
    let slug = path.into_inner();
    
    println!("Requisição para projeto com slug: {}", slug);
    
    if let Some(project) = read_project(&slug) {
        ctx.insert("project", &project);
        let rendered = tmpl.render("projects/show.html", &ctx)
            .map_err(|e| {
                println!("Erro ao renderizar template: {}", e);
                actix_web::error::ErrorInternalServerError("Template error")
            })?;
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        Ok(HttpResponse::NotFound().body(format!("Projeto não encontrado: {}", slug)))
    }
} 