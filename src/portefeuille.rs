use anyhow::Result;

pub fn parse(lines: std::str::Lines) -> Result<String> {
    let mut date = Err(anyhow::anyhow!("No operation date found"));
    let mut acc_amount = Err(anyhow::anyhow!("No account balance found"));

    let mut lines = lines.skip_while(|line| !line.trim().starts_with("AU "));

    // Date
    if let Some(line) = lines.next() {
        let pattern = "AU ";
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

    // Amount
    let mut lines = lines.skip_while(|line| {
        !line
            .trim()
            .starts_with("EVALUATION DE VOTRE PORTEFEUILLE :")
    });
    if let Some(line) = lines.next() {
        let pattern = "EVALUATION DE VOTRE PORTEFEUILLE : ";
        if let Some(pos) = line.find(pattern) {
            let line = &line[pos + pattern.len()..];
            acc_amount = Ok(line[pos..].trim());
        }
    }

    // Generate title
    Ok(format!(
        "{} Relevé de portefeuille (+ {})",
        date?, acc_amount?
    ))
}
