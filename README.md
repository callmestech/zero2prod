## Zero To Production in Rust

## Environment Variables

The following environment variables are used in the GitHub Actions workflow to deploy the Docker image to Amazon ECR:

- **AWS_ACCESS_KEY_ID**: The AWS access key ID for authenticating with AWS services.
- **AWS_SECRET_ACCESS_KEY**: The AWS secret access key for authenticating with AWS services.
- **AWS_REGION**: The AWS region where the ECR repository and ECS cluster are located.
- **AWS_ACCOUNT_ID**: The AWS account ID associated with the ECR repository.
- **ECR_REPOSITORY**: The name of the Amazon ECR repository where the Docker image will be pushed.
