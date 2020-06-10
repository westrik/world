use std::fs::File;
use std::io::Write;
use tectonic;

pub fn content_to_pdf(content: &str, output: Option<&str>) {
    // TODO: convert markdown content to latex
    let pdf_data: Vec<u8> = tectonic::latex_to_pdf(content).expect("processing failed");
    println!("Output PDF size is {} bytes.", pdf_data.len());
    if let Some(output_path) = output {
        println!("Writing to {}...", &output_path);
        // TODO: improve error handling
        let mut file = File::create(&output_path).unwrap();
        file.write_all(&pdf_data).unwrap();
    }
}

#[cfg(test)]
pub mod content_export {
    use super::*;

    #[test]
    fn latex_to_pdf() {
        let latex = r#"
    \documentclass{article}
    \begin{document}
    Hello, world!
    \end{document}
    "#;
        content_to_pdf(latex, None);
    }
}
