Run locally
```
brew install cargo-lambda
cargo lambda watch --env-file ../.env
```

Test running instance
```
cargo lambda invoke --data-ascii "{}"
```

Release
```
cargo lambda build --release --arm64
cargo lambda deploy --lambda-dir ../target/lambda
```
Then add env vars to a function. `cargo lambda` doesn't do in automatically.
Deploy process requires AWS CLI with IAM user having permissions:
```
{
  Effect = "Allow"
  Action = [
    "lambda:GetFunction",
    "lambda:CreateFunction",
    "lambda:UpdateFunctionCode",
    "lambda:UpdateFunctionConfiguration",
    "lambda:PublishVersion",
    "lambda:TagResource"
  ]
  Resource = [
    "arn:aws:lambda:<region>:<account-id>:function:<function-name>",
  ]
}
```
