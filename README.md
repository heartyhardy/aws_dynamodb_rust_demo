# AWS DynamoDB demo in Rust
#### This demo connects to a specified Amazon DynamoDB database and displays all items in a formatted table

## How to run this demo

1. Make sure Rust is installed
2. You need to have a properly configured AWS account with credentials saved to your user profile.
3. Make sure you have at least one DyanamoDB table setup in your AWS account
4. Clone the repo
5. Build the project using ```cargo build```
6. Run the project with specified flags. For example: ```cargo run -- -t "<TableName>" -r "<AWS region>" -i```

## Command like arguments

- ```-t --table "Table name goes here"``` : DynamoDB table name
- ```-r --region "Region name goes here"``` : AWS Region name
- ```-i --info``` : Show additional info

NOTE: This demo is based on the AWS-SDK for Rust examples.
