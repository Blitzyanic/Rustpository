pub fn ownership() {
    // String have ownership main pointer 
    let s: String = String::from("Hello!");
    let s2: String = String::from("world");

    // point to s no ownership
    let sp: &str = &s;

    // clone s, sc have ownership && sp/s are no longer existant
    // let sc: String = s.clone();

    // same as clone ownership by fs2 ( s2 -> ps -> fs2 )
    let fs2: String = test_ownership(s2);

    // point to s no ownership s is still avaidable
    point(&s);

    println!("{}, {}, {}", s, sp, fs2);
}

fn test_ownership(ps: String) -> String {
    println!("{}f", ps);

    return ps;
}

fn point(fsp: &String) {
    println!("{}", fsp);
}