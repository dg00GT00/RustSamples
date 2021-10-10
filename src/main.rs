struct ImportantExcept<'a> {
    par: &'a str,
}

impl<'a> ImportantExcept<'a> {
    fn announce_and_return_par(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.par
    }
}

fn main() {
    let t = ImportantExcept { par: "Something else" };
    let par = t.announce_and_return_par("Ora bolas");
    println!("{}", par);
}
