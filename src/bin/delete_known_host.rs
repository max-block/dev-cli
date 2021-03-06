use clap::{App, Arg};
use dns_lookup::lookup_host;

use dev_cli::VERSION;

fn main() {
    let matches = App::new("Delete hosts from ~/.ssh/known_hosts")
        .arg(Arg::with_name("hosts").multiple(true).required(true))
        .version(VERSION)
        .get_matches();

    for host in matches.values_of("hosts").unwrap().collect::<Vec<_>>() {
        delete_host(host);
    }
}

fn delete_host(host: &str) {
    dev_cli::shell_exec(format!("ssh-keygen -R {}", host).as_str());

    if let Ok(ips) = lookup_host(host) {
        for h in ips.iter() {
            dev_cli::shell_exec(format!("ssh-keygen -R {}", h.to_string()).as_str());
        }
    }
}
