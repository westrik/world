import aiohttp
from aiohttp import ClientSession
import asyncio
from collections import namedtuple
import json
import logging
import sys

logging.basicConfig(
    format="%(asctime)s %(levelname)s:%(name)s: %(message)s",
    level=logging.DEBUG,
    datefmt="%H:%M:%S",
    stream=sys.stderr,
)
logger = logging.getLogger("areq")
logging.getLogger("chardet.charsetprober").disabled = True

RequestSpec = namedtuple("RequestSpec", ["url", "method"])

SERVICE_TO_REQUEST_TO_SPEC = {
    "SENDGRID": {
        "SEND_EMAIL": RequestSpec("https://api.sendgrid.com/v3/mail/send", "POST")
    }
}


class ServiceProxyError(Exception):
    def __init__(self, message):
        self.message = message


async def send_request(session, request_spec, service_token, data, **kwargs):
    resp = await session.request(
        method=request_spec.method,
        url=request_spec.url,
        data=json.dumps(data),
        headers={
            "Content-Type": "application/json",
            "Authorization": "Bearer {}".format(service_token),
        },
        **kwargs
    )
    resp.raise_for_status()
    logger.info("Got response [%s] for URL: %s", resp.status, request_spec.url)


async def send_requests(request_spec, service_token, requests):
    async with ClientSession() as session:
        for idx, request in enumerate(requests):
            try:
                await send_request(session, request_spec, service_token, request)
            except (
                    aiohttp.ClientError,
                    aiohttp.http_exceptions.HttpProcessingError,
            ) as e:
                logger.error(
                    "aiohttp exception for %s [%s]: %s",
                    request_spec.url,
                    getattr(e, "status", None),
                    getattr(e, "message", None),
                )
                raise ServiceProxyError("Request #{} failed".format(idx))


def lambda_handler(event, context):
    service = event["service"]
    service_token = event["serviceToken"]
    request_type = event["requestType"]
    requests = event["requests"]
    try:
        request_spec = SERVICE_TO_REQUEST_TO_SPEC[service][request_type]
        asyncio.run(send_requests(request_spec, service_token, requests))
        return {"statusCode": 200}
    except (KeyError, ServiceProxyError) as e:
        return {"statusCode": 400, "body": str(e)}
