#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        for path in &["tests/testJPG.jpg", "tests/testHEIC.heic"] {
            let file = std::fs::File::open(path).expect("expected to open file");
             let mut bufreader = std::io::BufReader::new(&file);
             let exifreader = exif::Reader::new();
             let exif = exifreader.read_from_container(&mut bufreader).expect("could not read file?");
             println!("{}", path);
             for f in exif.fields() {
                 println!("{} {} {}",
                  f.tag, f.ifd_num, f.display_value().with_unit(&exif));
             }
             println!("\n\n");
        }
    }
}
