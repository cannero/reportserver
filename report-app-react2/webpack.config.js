var webpack = require('webpack');
var path = require('path');

var BUILD_DIR = path.resolve(__dirname, 'src/client/public');
var APP_DIR = path.resolve(__dirname, 'src/client/app');

//todo the environment variable is declared in start.js, test.js, build.js of react-scripts, used for babel-preset-react-app
process.env.NODE_ENV = 'development';

var config = {
    entry: APP_DIR + '/index.jsx',
    output: {
        path: BUILD_DIR,
        filename: 'bundle.js'
    },
    module: {
        loaders: [
            {
                test: /\.jsx?/,
                include: APP_DIR,
                loader: 'babel-loader',
                query: {
                    presets:['babel-preset-react-app']
                }
            },
            {
                test: /\.css$/,
                include: APP_DIR,
                loaders: ['style-loader', 'css-loader']
            },
            {
                test: /\.scss$/,
                include: APP_DIR,
                loaders: ['style-loader', 'css-loader', 'sass-loader']
            },
            {
                test: [/\.bmp$/, /\.gif$/, /\.jpe?g$/, /\.png$/],
                include: APP_DIR,
                loader: 'url-loader',
                options: {
                    limit: 10000,
                    name: 'static/[name].[hash:8].[ext]',
                },
            }
        ]
    }
};

module.exports = config;

