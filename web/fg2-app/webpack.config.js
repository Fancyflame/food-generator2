const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path')

module.exports = {
    entry: "./src/bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                'index.html',
                'dist/index.css',
                "cache.fg2"
            ]
        })
    ],
    experiments: {
        asyncWebAssembly: true
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: 'webassembly/async', // 使用 sync 加载 WebAssembly 文件
            }
        ]
    }
};
