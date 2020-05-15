import boto3
import certbot.main
import json
import os


def read_and_delete_file(path):
    with open(path, 'r') as file:
        contents = file.read()
    os.remove(path)
    return contents


def provision_cert(email, domains):
    print("Calling certbot to provision cert")
    try:
        response = certbot.main.main([
            'certonly',                # Obtain a cert but don't install it
            '-n',                      # Run in non-interactive mode
            '--agree-tos',             # Agree to the terms of service,
            '--email', email,          # Email
            '--dns-route53',           # Use dns challenge with route53
            '-d', ",".join(domains),   # Domains to provision certs for
            # Override directory paths so script doesn't have to be run as root
            '--config-dir', '/tmp/config-dir/',
            '--work-dir', '/tmp/work-dir/',
            '--logs-dir', '/tmp/logs-dir/',
        ])
        print("certbot returned {}".format(response))
    except Exception as e:
        print("certbot failed: {}".format(e))
        return None

    path = '/tmp/config-dir/live/' + domains[0] + '/'

    if os.path.exists(path + 'cert.pem'):
        return {
            'certificate': read_and_delete_file(path + 'cert.pem'),
            'private_key': read_and_delete_file(path + 'privkey.pem'),
            'certificate_chain': read_and_delete_file(path + 'chain.pem')
        }
    else:
        return None


def upload_cert_to_secrets_manager(secret_id, domains, cert):
    client = boto3.client('secretsmanager')
    client.put_secret_value(
        SecretId=secret_id,
        SecretString=json.dumps({"certificate": cert, "domains": domains}),
    )


def maybe_provision(domains, email, secret_id):
    cert = provision_cert(email, domains)
    if cert:
        upload_cert_to_secrets_manager(secret_id, domains, cert)
        return True
    return False


def lambda_handler(event, context):
    provisioned = maybe_provision(**event)

    return {
        "statusCode": 200,
        "body": json.dumps({
            "status": "provisioned or renewed certificate" if provisioned else "renewal not needed",
        }),
    }
