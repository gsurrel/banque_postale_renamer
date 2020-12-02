use anyhow::Result;

pub fn parse(lines: std::str::Lines) -> Result<String> {
    let mut date = Err(anyhow::anyhow!("No operation date found"));
    let mut acc_name = Err(anyhow::anyhow!("No account name found"));
    let mut acc_amount = Err(anyhow::anyhow!("No account balance found"));

    let mut lines = lines.skip_while(|line| !line.trim().starts_with("Situation de votre"));

    // Account name
    if let Some(line) = lines.next() {
        if let Some(pos) = line.find(" n° ") {
            acc_name = Ok(line[19..pos].trim());
        }
    }

    let mut lines = lines.skip(2);

    // Split line properly
    if let Some(line) = lines.next() {
        let pattern = "solde au ";
        if let Some(pos) = line.find(pattern) {
            let val = &line[pos + pattern.len()..];
            let parse_date = chrono::NaiveDate::from_ymd(
                val[6..10].parse().unwrap(),
                val[3..5].parse().unwrap(),
                val[0..2].parse().unwrap(),
            );
            date = Ok(parse_date);

            acc_amount = Ok(line[pos + 20..line.len() - 2].trim());
        }
    }

    // Find a possible date (in the CPP)
    for line in lines {
        if date.is_err() {
            let pattern = "> Découvert autorisé au ";
            if let Some(pos) = line.find(pattern) {
                let val = &line[pos + pattern.len()..];
                let parse_date = chrono::NaiveDate::from_ymd(
                    val[6..10].parse().unwrap(),
                    val[3..5].parse().unwrap(),
                    val[0..2].parse().unwrap(),
                );
                date = Ok(parse_date);
            }
        }
    }

    // Generate title
    Ok(format!(
        "{} Relevé de compte {} ({} EUR)",
        date?, acc_name?, acc_amount?
    ))
}
