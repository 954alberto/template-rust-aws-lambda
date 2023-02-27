async fn handler(event: LambdaEvent<Value>) -> Result<(), SdkError<AttachRolePolicyError>> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = service_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

/*
#[cfg(test)]
mod test {
    use lambda_runtime::{Context, LambdaEvent};
    #[tokio::test]
    async fn test() {
    }
}
*/
