extern crate reqwest;

mod source;

use source::Source;
use source::Download;

fn main() {
    let sources = vec![
        Source {
            name: String::from("Harsh Shandilya's hosts list"),
            url: String::from("https://download.msfjarvis.website/adblock/hosts")
        }
    ];
    let mut hosts = String::new();
    for source in sources {
        hosts.push_str(source.download_to_string().unwrap().as_str());
    }
    println!("{}", hosts);
}
