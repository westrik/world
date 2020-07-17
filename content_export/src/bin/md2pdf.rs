use std::{env, io, process};

use world_content_export::content_to_pdf;

const LATEX_PRELUDE: &str = r#"
\documentclass{article}
\usepackage[colorlinks, allcolors=blue]{hyperref}
\usepackage{tikz}
\usetikzlibrary{positioning}
"#;

const SAMPLE_DIAGRAM: &str = r#"
\section{TikZ test}

\begin{tikzpicture}[every node/.style={minimum size=1cm},on grid]
\begin{scope}[every node/.append style={yslant=-0.5},yslant=-0.5]
  \shade[right color=gray!10, left color=black!50] (0,0) rectangle +(3,3);
  \node at (0.5,2.5) {9};
  \node at (1.5,2.5) {7};
  \node at (2.5,2.5) {1};
  \node at (0.5,1.5) {2};
  \node at (1.5,1.5) {4};
  \node at (2.5,1.5) {8};
  \node at (0.5,0.5) {5};
  \node at (1.5,0.5) {3};
  \node at (2.5,0.5) {6};
  \draw (0,0) grid (3,3);
\end{scope}
\begin{scope}[every node/.append style={yslant=0.5},yslant=0.5]
  \shade[right color=gray!70,left color=gray!10] (3,-3) rectangle +(3,3);
  \node at (3.5,-0.5) {3};
  \node at (4.5,-0.5) {9};
  \node at (5.5,-0.5) {7};
  \node at (3.5,-1.5) {6};
  \node at (4.5,-1.5) {1};
  \node at (5.5,-1.5) {5};
  \node at (3.5,-2.5) {8};
  \node at (4.5,-2.5) {2};
  \node at (5.5,-2.5) {4};
  \draw (3,-3) grid (6,0);
\end{scope}
\begin{scope}[every node/.append style={
    yslant=0.5,xslant=-1},yslant=0.5,xslant=-1
  ]
  \shade[bottom color=gray!10, top color=black!80] (6,3) rectangle +(-3,-3);
  \node at (3.5,2.5) {1};
  \node at (3.5,1.5) {4};
  \node at (3.5,0.5) {7};
  \node at (4.5,2.5) {5};
  \node at (4.5,1.5) {6};
  \node at (4.5,0.5) {8};
  \node at (5.5,2.5) {2};
  \node at (5.5,1.5) {3};
  \node at (5.5,0.5) {9};
  \draw (3,0) grid (6,3);
\end{scope}
\end{tikzpicture}
"#;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: md2pdf [output_filename]");
        process::exit(1);
    }
    let mut content = LATEX_PRELUDE.to_string();
    content.push_str(
        r#"
\title{Test Document}
\author{Bernard Lowe}
\date{December 7, 1965}
\begin{document}
\maketitle
Hello world!!!
$$x=y^2$$
    "#,
    );
    content.push_str(SAMPLE_DIAGRAM);
    content.push_str(r#"\end{document}"#);
    content_to_pdf(&content, Some(&args[1]));

    Ok(())
}
