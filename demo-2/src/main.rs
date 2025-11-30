fn main() {
    print!("Hello, world!  1\t");
    print!("Hello, world!  2\n");
    print!("Hello, world!  2\"d\"");

    eprintln!("error tesr");

    let name = "shadmehr";
    let family = "yadani";
    let age = 24;
    let agestring = format!("\nmy name is {name}  {family} age= {age}");
    print!("{}", agestring)
}
