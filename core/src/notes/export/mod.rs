pub mod html;
pub mod latex;
pub mod markdown;

pub trait Render<T> {
    fn render(&self) -> T;
}
