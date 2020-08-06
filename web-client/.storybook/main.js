const TsconfigPathsPlugin = require('tsconfig-paths-webpack-plugin');

const path = require('path');

module.exports = {
    stories: ['../src/**/*.stories.[tj]sx'],
    webpackFinal: (config) => {
        console.dir(config, { depth: null });
        config.module.rules.push({
            test: /\.tsx?$/,
            loader: 'ts-loader',
            exclude: /node_modules/,
        });
        config.resolve.extensions.push('.ts', '.tsx');
        config.resolve.alias['~'] = path.resolve(__dirname, '../src/');
        config.resolve.alias['tests'] = path.resolve(__dirname, '../tests/');
        config.resolve.plugins = [new TsconfigPathsPlugin({ configFile: path.resolve(__dirname, '../tsconfig.json') })];
        return config;
    },
};
