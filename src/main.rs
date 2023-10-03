use yargs::*;

fn main() {
    let mut ag = ArgParser::new();
    ag.add_flag("debug".into(), None, Some("some help".into()), false, false);
    ag.add_option("name".into(), Some('n'), None, None, None, None, true);
    let res = ag.parse();
    if res.is_err() {
        println!("ERROR");
        println!("{:?}", res.unwrap_err());
        return;
    }
    println!("{:?}", res.unwrap());
}
