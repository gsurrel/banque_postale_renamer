use anyhow::{Result, Context};
use walkdir::WalkDir;

mod compte_rendu_operation;
mod portefeuille;
mod releve_de_compte;
mod releve_de_comptes;
mod releve_de_pea;

#[derive(Debug)]
enum Type {
    Unknown,
    CompteRenduOperation,
    ReleveDeComptes,
    ReleveDeCompte,
    ReleveDePEA,
    Portefeuille,
}

fn main() -> Result<()> {
    println!("Renames all recognized PDF files from La Banque Postale in the current path");
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().extension().is_some())
        .filter(|e| e.path().extension().unwrap() == "pdf")
    {
        println!("{}", entry.path().display());
        let text = std::process::Command::new("pdftotext")
            .arg("-layout")
            .arg(entry.path())
            .arg("-")
            .output().context("Failed to run 'pdftotext'")?
            .stdout;
        let text = String::from_utf8(text).context("Failed to read the output of 'pdftotext'")?;
        let mut lines = text.lines();

        let doc_type;
        loop {
            if let Some(line) = lines.next() {
                let line = line.trim_start();
                if line.starts_with("COMPTE RENDU D'OPERATION") {
                    doc_type = Type::CompteRenduOperation;
                    break;
                } else if line.starts_with("Relevé de vos comptes") {
                    doc_type = Type::ReleveDeComptes;
                    break;
                } else if line.starts_with("Relevé de votre") {
                    doc_type = Type::ReleveDeCompte;
                    break;
                } else if line.starts_with("VOTRE PORTEFEUILLE") {
                    doc_type = Type::Portefeuille;
                    break;
                } else if line.starts_with("VOTRE RELEVE PEA") {
                    doc_type = Type::ReleveDePEA;
                    break;
                }
            } else {
                doc_type = Type::Unknown;
                break;
            }
        }

        let title = match doc_type {
            Type::Unknown => None,
            Type::CompteRenduOperation => compte_rendu_operation::parse(lines).ok(),
            Type::ReleveDeComptes => releve_de_comptes::parse(lines).ok(),
            Type::ReleveDeCompte => releve_de_compte::parse(lines).ok(),
            Type::ReleveDePEA => releve_de_pea::parse(lines).ok(),
            Type::Portefeuille => portefeuille::parse(lines).ok(),
        };

        if let Some(mut title) = title {
            title += ".pdf";
            println!("{}", title);

            std::fs::rename(entry.path(), title).context("Failed to rename file {}", entry.path().display())?;
        } else {
            println!("Document not recognized");
        }
    }

    Ok(())
}
