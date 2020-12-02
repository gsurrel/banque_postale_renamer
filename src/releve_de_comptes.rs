use anyhow::Result;

struct Account {
    name: String,
    amount: String,
}

pub fn parse(lines: std::str::Lines) -> Result<String> {
    let mut date = Err(anyhow::anyhow!("No operation date found"));
    let mut accounts = vec![];

    // Skip lines until account summary
    let mut lines = lines.skip_while(|line| !line.trim().starts_with("Situation de vos comptes"));
    lines.next();

    // Iterate over accounts
    while let Some(line) = lines.next() {
        if let Some(pos) = line.find(" n° ") {
            let name = line[0..pos].trim().to_string();
            let amount = line[pos + 30..].trim();
            let amount = amount[..amount.len() - 2].trim().to_string();
            accounts.push(Account { name, amount });
        } else {
            break;
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
    let mut out = format!("{} Relevé de comptes", date.unwrap());
    for account in accounts {
        out += &format!(" - {} ({} EUR)", account.name, account.amount);
    }

    Ok(out)
}
