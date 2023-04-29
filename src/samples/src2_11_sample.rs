#[allow(dead_code)]
pub fn src2_11() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 132, 203, 877, 4140, 21147];

    for item in &haystack {
        if *item == needle {
            println!("{}", item);
        }
    }
}

#[allow(dead_code)]
pub fn src2_11_2() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5, 15, 52, 132, 203, 877, 4140, 21147];

    for item in haystack {
        if item == needle {
            println!("{}", item);
        }
    }
}

#[allow(dead_code)]
pub fn src2_11_3() {
    let needle = "132";
    let haystack = ["1", "1", "2", "5", "15", "52", "132", "203", "877", "4140", "21147"];

    for item in &haystack {
        if *item == needle {
            println!("{}", *item);
        }
    }
}

#[allow(dead_code)]
pub fn src2_11_4() {
    let needle = "132";
    let haystack = ["1", "1", "2", "5", "15", "52", "132", "203", "877", "4140", "21147"];

    for item in haystack {
        if item == needle {
            println!("{}", item);
        }
    }
}
