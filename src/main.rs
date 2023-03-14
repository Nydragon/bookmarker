use std::{error::Error, fs::File, path::PathBuf};
#[derive(Debug, serde::Deserialize)]
struct Record {
    id: i128,
    alias: Option<String>,
    content: String,
}

fn read() -> Result<Vec<Record>, Box<dyn Error>> {
    use csv::Reader;

    let home: PathBuf = dirs::home_dir().unwrap();
    let path: PathBuf = home.join(".config/bookmarker-rs/bookmarks.csv");

    let mut rdr: Reader<File> = csv::Reader::from_path(path).expect("");

    let mut data: Vec<Record> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        data.push(record);
    }
    return Ok(data);
}

fn create_prompt(data: &Vec<Record>) -> Vec<String> {
    let options: Vec<String> = data
        .iter()
        .map(|rec| {
            if let Some(alias) = &rec.alias {
                format!("{}: {}", rec.id, alias)
            } else {
                format!("{}: {}", rec.id, rec.content)
            }
        })
        .collect();

    return options;
}

fn get_selected_content(selection: String) -> i128 {
    use regex::{Captures, Regex};

    let re: Regex = Regex::new(r"^\d").unwrap();

    let res: Captures = re.captures(&selection).unwrap();

    if let Some(res) = res.get(0) {
        return String::from(res.as_str()).parse::<i128>().unwrap();
    } else {
        return -1;
    }
}

fn write_selection(selection: &Record) {
    use enigo::Enigo;
    use enigo::KeyboardControllable;

    let mut enigo: Enigo = Enigo::new();

    enigo.key_sequence_parse(&selection.content);
}

fn main() {
    use simple_dmenu::dmenu;

    let data: Vec<Record> = read().expect("CSV to exist.");

    let options: Vec<String> = create_prompt(&data);

    let output: String = dmenu!(iter options; args "-i", "-l", "50");

    let selected_id: i128 = get_selected_content(output);

    let selection: Option<&Record> = data.iter().find(|rec| rec.id == selected_id);

    if let Some(selection) = selection {
        write_selection(selection);
    }
}
