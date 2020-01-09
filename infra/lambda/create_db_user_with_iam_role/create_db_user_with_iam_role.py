import pg8000
# import boto3
# import os
# import base64
import json

def lambda_handler(event, context):
    # sm = boto3.client('secretsmanager')
    # response = sm.get_secret_value(SecretId='db_password_westrikworld_app')
    # if 'SecretString' in response:
    #     info = response['SecretString']
    # else:
    #     info = base64.b64decode(response['SecretBinary'])
    # for now, get secret from env. TODO: get from secretsmanager

    database = event['database']
    conn = pg8000.core.Connection(
        application_name='lambda-create_db_user_with_iam_role',
        user=event['username'],
        host=event['host'],
        port=int(event['port']),
        database=database,
        password=event['password'],
        ssl=False,
        unix_sock=None,
        timeout=1,
        max_prepared_statements=100,
        tcp_keepalive=True
    )
    cursor = conn.cursor()
    try:
        cursor.execute("SELECT 1 FROM pg_user WHERE usename = 'westrikworld_app'")
        if len(list(cursor)):
            return {"status":"exists"}
        else:
            cursor.execute("CREATE USER westrikworld_app with login createdb")
            conn.commit()
            cursor.execute("GRANT rds_iam TO westrikworld_app")
            conn.commit()
            cursor.execute("ALTER DATABASE %s OWNER TO westrikworld_app" % database)
            conn.commit()
            return {"status":"created"}
    finally:
        conn.close()
