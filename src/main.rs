struct Error1 {}

struct Error2 {}

impl From<Error1> for Error2 {
    #[track_caller]
    fn from(_: Error1) -> Error2 {
        println!("{:?}", std::panic::Location::caller());
        Error2 {}
    }
}

fn error1() -> Result<(), Error1> {
    Err(Error1 {})
}

fn error2() -> Result<(), Error2> {
    let res = error1()?;
    Ok(res)
}

fn main() {
    error2().unwrap_err();
}
