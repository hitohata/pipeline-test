import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as gateway from 'aws-cdk-lib/aws-apigateway';
import { RustFunction } from 'cargo-lambda-cdk';
import { join } from 'path';

export class APIStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const someFunction = new RustFunction(this, 'someFunction', {
      functionName: "some-function",
      manifestPath: join(__dirname, '../../lambdas/some-function/Cargo.toml'),
      runtime: "provided.al2023"
    });

    new gateway.LambdaRestApi(this, 'someApi', {
      handler: someFunction,
    })
  }
}
