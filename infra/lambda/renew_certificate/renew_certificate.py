import boto3
import certbot.main
import datetime
import os


def read_and_delete_file(path):
    with open(path, 'r') as file:
        contents = file.read()
    os.remove(path)
    return contents


def provision_cert(email, domains):
    certbot.main.main([
        'certonly',                             # Obtain a cert but don't install it
        '-n',                                   # Run in non-interactive mode
        '--agree-tos',                          # Agree to the terms of service,
        '--email', email,                       # Email
        '--dns-route53',                        # Use dns challenge with route53
        '-d', domains,                          # Domains to provision certs for
        # Override directory paths so script doesn't have to be run as root
        '--config-dir', '/tmp/config-dir/',
        '--work-dir', '/tmp/work-dir/',
        '--logs-dir', '/tmp/logs-dir/',
    ])

    first_domain = domains.split(',')[0]
    path = '/tmp/config-dir/live/' + first_domain + '/'
    return {
        'certificate': read_and_delete_file(path + 'cert.pem'),
        'private_key': read_and_delete_file(path + 'privkey.pem'),
        'certificate_chain': read_and_delete_file(path + 'chain.pem')
    }


def should_provision(secret_id, domains):
    existing_cert = find_existing_cert(secret_id, domains)
    if existing_cert:
        now = datetime.datetime.now(datetime.timezone.utc)
        not_after = existing_cert['Certificate']['NotAfter']
        return (not_after - now).days <= 30
    else:
        return True


def find_existing_cert(secret_id, domains):
    _domains = frozenset(domains)
    client = boto3.client('secretsmanager')
    response = client.get_secret_value(SecretId=secret_id)
    # TODO: check what domains are on the cert, maybe return None
    return response


def upload_cert_to_secrets_manager(secret_id, domains, cert):
    # TODO: upload to AWS Secrets Manager
    client = boto3.client('secretsmanager')
    response = client.put_secret_value(
        SecretId=secret_id,
    #     ClientRequestToken='string',
    #     SecretBinary=b'bytes',
        SecretString='string',
    #     VersionStages=[
    #         'string',
    #     ]
    )
    pass


def maybe_provision(domains, email, secret_id):
    if should_provision(secret_id, domains):
        cert = provision_cert(email, domains)
        upload_cert_to_secrets_manager(secret_id, domains, cert)


def lambda_handler(event, context):
    maybe_provision(**event)
