use anyhow::Result;
use string_morph::Morph;

pub fn parse(lines: std::str::Lines) -> Result<String> {
    let mut col_credit = Err(anyhow::anyhow!("Column header 'credit' not found"));
    let mut op_name = Err(anyhow::anyhow!("No operation name found"));
    let mut op_type = Err(anyhow::anyhow!("No operation type found"));
    let mut op_libelle = Err(anyhow::anyhow!("No libelle found"));
    let mut op_quantite = Err(anyhow::anyhow!("No quantity found"));
    let mut op_date = Err(anyhow::anyhow!("No operation date found"));
    let mut op_balance = Err(anyhow::anyhow!("No operation balance found"));

    let mut rachats_et_souscriptions = false;

    for line in lines {
        // Find the 'CREDIT' column offset
        if line.ends_with("CREDIT") && col_credit.is_err() {
            if let Some(pos) = line.rfind("CREDIT") {
                col_credit = Ok(pos);
            }
        }

        // Find a possible date
        if op_date.is_err() {
            if let Some(pos) = line.rfind("date du") {
                let val = &line[pos + 8..];
                let date = chrono::NaiveDate::from_ymd(
                    val[6..10].parse().unwrap(),
                    val[3..5].parse().unwrap(),
                    val[0..2].parse().unwrap(),
                );
                op_date = Ok(date);
            }
        }

        // Special "Rachat et souscriptions"
        if !rachats_et_souscriptions && line.contains("RACHATS ET SOUSCRIPTIONS") {
            rachats_et_souscriptions = true;
            op_type = Ok("Rachat et souscriptions".to_string());
        }
        if rachats_et_souscriptions {
            if op_balance.is_err() && line.starts_with("TOTAUX") {
                let credit = line[*col_credit.as_ref().unwrap()..].trim();
                op_balance = Ok(format!("+{}", credit));
            }
        }

        // Find ":" is exists
        if let Some(pos) = line.rfind(":") {
            // Split the line
            let key = &line[0..pos].trim_start();
            let val = &line[pos + 1..].trim();
            // Extract what we are looking for
            if key.starts_with("NATURE D'OPERATION") {
                op_name = Ok(val.to_sentence_case());
            } else if key.starts_with("OPERATION") {
                op_type = Ok(val.to_sentence_case());
            } else if key.starts_with("LIBELLE VALEUR") {
                op_libelle = Ok(val.to_string());
            } else if key.starts_with("QUANTITE") {
                op_quantite = Ok(val.to_string());
            } else if key.starts_with("Date") {
                let date = chrono::NaiveDate::from_ymd(
                    val[6..10].parse().unwrap(),
                    val[3..5].parse().unwrap(),
                    val[0..2].parse().unwrap(),
                );
                op_date = Ok(date);
            } else if key.starts_with("NET") {
                if let Some(pos_eur) = val.rfind("EUR") {
                    let val = &val[pos_eur + 3..].trim();
                    if line.len() > *col_credit.as_ref().unwrap() {
                        op_balance = Ok(format!("+{}", val));
                    } else {
                        op_balance = Ok(format!("-{}", val));
                    }
                } else {
                    println!("Could not find EURO text!")
                }
            }
        }
    }

    let mut out = format!("{} Compte rendu opération", op_date?);
    if let Ok(val) = op_name {
        out += &format!(" - {}", val);
    }
    if let Ok(val) = op_type {
        out += &format!(" - {}", val);
    }
    if let Ok(val) = op_libelle {
        out += &format!(" '{}'", val);
    }
    if let Ok(val) = op_balance {
        out += &format!(" ({} EUR)", val);
    }
    if let Ok(val) = op_quantite {
        out += &format!(" (#{})", val);
    }

    Ok(out)
}
