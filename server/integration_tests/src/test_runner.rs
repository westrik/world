#[cfg(test)]
pub fn runner(ts: &[&dyn Fn()]) {
    println!("Running {} tests: ", ts.len());
    for t in ts {
        println!("Running test");
        t();
    }
}
