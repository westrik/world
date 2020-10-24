const path = require('path');
const TsconfigPathsPlugin = require('tsconfig-paths-webpack-plugin');

module.exports = {
    stories: ['../src/**/*.stories.[tj]sx'],
    webpackFinal: (config) => {
        config.module.rules.push(
            {
                test: /\.s[ac]ss$/i,
                use: [
                    'style-loader',
                    'css-loader',
                    'sass-loader',
                ],
            },
            {
                test: /\.tsx?$/,
                loader: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.svg$/,
                use: [
                    {
                        loader: 'babel-loader',
                        options: {
                            presets: ['preact', 'env'],
                        },
                    },
                    {
                        loader: '@svgr/webpack',
                        options: {
                            babel: false,
                            icon: true,
                            expandProps: false,
                            typescript: true
                        },
                    },
                ],
            },
        );
        config.resolve.extensions.push('.ts', '.tsx');
        config.resolve.plugins = [
            new TsconfigPathsPlugin({
                configFile: path.resolve(__dirname, '../tsconfig.json')
            })
        ];
        return config;
    },
};
