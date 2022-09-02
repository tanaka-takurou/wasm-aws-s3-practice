# s3-webassembly kit
Simple kit for webassembly web page using S3.


## Requirements
- aws-sam-cli
- rust environment


## Usage

### Deploy
```bash
sam package --output-template-file packaged.yml --s3-bucket {bucket}
sam deploy --stack-name {stack} --capabilities CAPABILITY_IAM --template-file packaged.yml
```
