# Create a powerful Tokenizer API using distroless

By using distroless, you ensure a low weight for your container. This full example will get you everything you need to run an API in a container that you can then use to deploy to a cloud provider like Azure.

This repository gives you a good starting point with a Dockerfile, GitHub Actions workflow, and Rust code.

## Generate a PAT

The access token will need to be added as an Action secret. [Create one](https://github.com/settings/tokens/new?description=Azure+Rust+Container+Apps+access&scopes=write:packages) with enough permissions to write to packages.

## Create an Azure Service Principal

You'll need the following:

1. An Azure subscription ID [find it here](https://portal.azure.com/#view/Microsoft_Azure_Billing/SubscriptionsBlade) or [follow this guide](https://docs.microsoft.com/en-us/azure/azure-portal/get-subscription-tenant-id)
1. A Service Principal with the following details the AppID, password, and tenant information. Create one with: `az ad sp create-for-rbac -n "REST API Service Principal"` and assign the IAM role for the subscription. Alternatively set the proper role access using the following command (use a real subscription id and replace it):

```
az ad sp create-for-rbac --name "CICD" --role contributor --scopes /subscriptions/$AZURE_SUBSCRIPTION_ID --sdk-auth
``` 

## Azure Container Apps

Make sure you have one instance already created, and then capture the name and resource group. These will be used in the workflow file.

## No need to change defaults 

Unlike other language runtimes like Python, you don't need to change the default container service. Rust will be more than happy to use a single CPU!

## Gotchas

There are a few things that might get you into a failed state when deploying:

* Not using authentication for accessing the remote registry (ghcr.io in this case). Authentication is always required
* Not using a PAT (Personal Access Token) or using a PAT that doesn't have write permissions for "packages".
* Different port than 80 in the container. By default Azure Container Apps use 80. Update to match the container.

If running into trouble, check logs in the portal or use the following with the Azure CLI:

```
az containerapp logs  show  --name $CONTAINER_APP_NAME --resource-group $RESOURCE_GROUP_NAME --follow
```

Update both variables to match your environment

## API Best Practices

Although there are a few best practices for using the FastAPI framework, there are many different other suggestions to build solid HTTP APIs that can be applicable anywhere. 

### Use HTTP Error codes
The HTTP specification has several error codes available. Make use of the appropriate error code to match the condition that caused it. For example the `401` HTTP code can be used when access is unauthorized. You shouldn't use a single error code as a catch-all error.

Here are some common scenarios associated with HTTP error codes:

- `400 Bad request`: Use this to indicate a schema problem. For example if the server expected a string but got an integer
- `401 Unauthorized`: When authentication is required and it wasn't present or satisfied
- `404 Not found`: When the resource doesn't exist

Note that it is a good practice to use `404 Not Found` to protect from requests that try to find if a resource exists without being authenticated. A good example of this is a service that doesn't want to expose usernames unless you are authenticated.


### Accept request types sparingly

| GET | POST | PUT | HEAD|
|---|---|---|---|
| Read Only | Write Only | Update existing | Does it exist? |
