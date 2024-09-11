const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
    entry: './src/index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js'
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./src/index.html"
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "..")
        }),
        new CopyWebpackPlugin({
            patterns: [
                { from: './assets', to: 'assets' },
            ],
        })
    ],
    devServer: {
        static: {
            directory: path.resolve(__dirname, './assets'),
                publicPath: '/assets'
        }
    },
    mode: 'development',
    experiments: {
        asyncWebAssembly: true
    }
};