export const SITE_PROPS = { siteName: 'westrikworld' };

const API_HOSTS = {
  local: 'http://api.westrik.world:6874',
  production: 'https://api.westrikworld.com',
  staging: 'https://api.staging.westrikworld.com',
};

const env = process.env.NODE_ENV;
export const API_HOST =
  env === 'staging'
    ? API_HOSTS.staging
    : env === 'production'
    ? API_HOSTS.production
    : API_HOSTS.local;
