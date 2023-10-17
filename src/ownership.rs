pub fn ownership() {
    // String have ownership main pointer 
    let s: String = String::from("Hello!");
    let s2: String = String::from("world");

    // point to s no ownership
    let sp: &str = &s;

    // clone sc s sc have ownership && sp are no longer existant
    // let sc: String = s.clone();

    // same as clone ownership by fs2 ( s2 -> ps -> fs2 )
    let fs2: String = test_ownership(s2);

    // point to s no ownership s is still avaidable
    point(&s);

    print!("{}, {}, {}", s, sp, fs2);
}

fn test_ownership(ps: String) -> String {
    print!("{}f", ps);

    return ps;
}

fn point(fsp: &String) {
    print!("{}", fsp);
}