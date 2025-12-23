use crate::scan::discover::DiscoveredClass;
use tabled::Table;
use tabled::settings::Style;

pub(crate) fn print_unused_classes_as_table(classes: Vec<DiscoveredClass>) {
    let mut table = Table::new(classes);
    table.with(Style::rounded());
    println!("{}", table);
}

pub(crate) fn print_unused_classes_as_json(classes: Vec<DiscoveredClass>) {
    if let Ok(json) = serde_json::to_string_pretty(&classes) {
        println!("{}", json);
    } else {
        println!("Failed to serialize classes to JSON");
    }
}
