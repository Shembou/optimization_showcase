import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
import HtmlWebpackPlugin from 'html-webpack-plugin';
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import MiniCssExtractPlugin from 'mini-css-extract-plugin';

const config = {
    entry: './src/index.tsx',

    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'bundle.js',
        publicPath: '/',
        clean: true,
    },

    plugins: [
        new HtmlWebpackPlugin({
            template: './src/index.html',
            filename: 'index.html',
        }),
        new MiniCssExtractPlugin({
            filename: 'styles.css'
        }),
    ],

    mode: 'development',

    experiments: {
        asyncWebAssembly: true
    },

    module: {
        rules: [
            {
                test: /\.(ts|tsx)$/,
                exclude: /node_modules/,
                use: {
                    loader: 'babel-loader',
                    options: {
                        presets: [
                            '@babel/preset-env',
                            '@babel/preset-typescript',
                            ['@babel/preset-react', { runtime: 'automatic' }] // Automatic JSX runtime
                        ]
                    }
                }
            },

            {
                test: /\.css$/i,
                use: [MiniCssExtractPlugin.loader, 'css-loader', 'postcss-loader'], // Added postcss-loader for Tailwind
            },

            {
                test: /\.(png|svg|jpg|jpeg|gif)$/i,
                type: 'asset/resource',
            },

            {
                test: /\.(woff|woff2|eot|ttf|otf)$/i,
                type: 'asset/resource',
            },
        ],
    },

    resolve: {
        extensions: ['.ts', '.tsx', '.json', ".wasm", ".js", ".jsx"], // Allow importing .jsx files without extension
    },

    devServer: {
        historyApiFallback: true, // Serve index.html for all routes (SPA routing)
        hot: true,
        open: true,
        port: 3000,
        static: {
            directory: path.join(__dirname, 'dist'),
        },
    }
};

export default config;