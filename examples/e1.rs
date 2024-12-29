use deep_flatten2::OptionDeepFlatten;
use deep_flatten2::ResultDeepOk;

fn main() {
    let x: Result<Result<i32, &str>, &str> = Ok(Ok(1));
    let y: Result<Result<Result<i32, &str>, &str>, &str> = Ok(Ok(Ok(1)));

    assert_eq!(x.deep_ok(), Some(1));
    assert_eq!(y.deep_ok(), Some(1));

    let x = Some(Some(Some(Some(Some(Some(Some(Some(Some(())))))))));
    let flattened = x.deep_flatten();

    assert_eq!(flattened, Some(()));
}
