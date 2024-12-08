fn main() {
    pkg_config::Config::new()
        .exactly_version("1.5.8")
        .probe("babeltrace")
        .unwrap();

    pkg_config::Config::new()
        .exactly_version("1.5.8")
        .probe("babeltrace-ctf")
        .unwrap();
}
