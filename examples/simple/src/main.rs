#[macro_use]
extern crate try_block;

fn do_one(x: i32) -> Result<i32, ()> {
    Ok(x)
}

fn do_two(x: i32) -> Result<i32, ()> {
    Err(())
}

fn main() {
    let x = 5;
    let result: Result<_, ()> = try_block! {
        let a = do_one(x)?;
        let b = do_two(a)?;
        Ok(b)
    };

    println!("{:?}", result);
}
