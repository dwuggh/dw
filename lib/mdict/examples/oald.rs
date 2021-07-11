use mdict::mdict::MDict;

fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let mdd_file_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdd";
    let mdx_file_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdx";
    // let mdx_file_path = "/home/dwuggh/.dicts/hanying/汉英词典（第三版）.mdd";
    let mdict = MDict::new(mdx_file_path, Some(mdd_file_path.to_string()))?;
    println!("{}", mdict.words.len());
    println!("{}", mdict.lookup("fuck").unwrap());
    // println!("{:?}", mdict.lookup_assets("oalecd9.css"));
    Ok(())
}
