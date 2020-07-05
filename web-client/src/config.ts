// magic to make enzyme + preact types work:
/// <reference types="enzyme-adapter-preact-pure" />

const env = process.env.NODE_ENV;

export const SITE_NAME = 'westrikworld';

const API_HOSTS = {
    local: 'https://api.local.westrik.world',
    staging: 'https://api.staging.westrik.world',
    production: 'https://api.westrikworld.com',
};
let host = API_HOSTS.local;
if (env === 'staging') {
    host = API_HOSTS.staging;
}
if (env === 'production') {
    host = API_HOSTS.production;
}
export const API_HOST = host;

const config = {
    API_HOST,
    SITE_NAME,
};
export default config;
