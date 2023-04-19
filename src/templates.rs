use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new("src/templates/**/*.html").expect("Failed to import templates");
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}
