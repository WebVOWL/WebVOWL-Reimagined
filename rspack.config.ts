// Rspack config file - A Rust-based drop-in replacement for Webpack
// https://rspack.rs/config/

import { defineConfig } from "@rspack/cli";
import { rspack } from "@rspack/core";
import { CleanWebpackPlugin } from "@webvowl/clean-webpack-plugin";
import WasmPackPlugin from "@webvowl/wasm-pack-plugin";
import CompressionPlugin from "compression-webpack-plugin";
import HtmlMinimizerPlugin from "html-minimizer-webpack-plugin";
import zlib from "node:zlib";
import path from "path";
import GlobalPaths from "./global-paths.ts";
const __dirname = import.meta.dirname;
const pkg = await import("./package.json", { with: { type: "json" } });

/**Is true if production build is enabled and false otherwise.*/
const PROD = process.env.NODE_ENV === "production";
const NAME = (function () {
    // Formats the pkg.name into a pretty string
    // E.g. "webvowl-reimagined" ---> "WebVOWL Reimagined"
    let words = pkg.name.replace("-", " ").split(" ");
    for (let i = 0; i < words.length; i++) {
        if (words[i].toUpperCase() == "WEBVOWL") {
            words[i] = "WebVOWL";
            continue;
        }
        words[i] = words[i][0].toLowerCase() + words[i].substring(1);
    }
    return words.join(" ");
})();

/** Main config */
export default defineConfig({
    target: "web",
    cache: !PROD,
    // @ts-ignore
    mode: process.env.NODE_ENV,
    devtool: PROD ? false : "source-map",
    entry: {
        app: {
            import: [`./src/test/test.js`], // FIXME: Testing
        },
    },
    output: {
        path: path.resolve(__dirname, GlobalPaths.deploy),
        publicPath: "auto",
        filename: "js/[name].js",
        chunkFilename: "js/[chunkhash].js",
        webassemblyModuleFilename: "wasm/[id].[hash].wasm",
        // enabledWasmLoadingTypes: ['fetch'],
        // workerChunkLoading: "universal",
        // globalObject: 'this',
        // module: true,
        // library: {
        // 	name: 'webvowl',
        // 	type: 'umd',
        // },
    },
    optimization: {
        minimize: PROD,
        minimizer: [
            // Minify JS
            new rspack.SwcJsMinimizerRspackPlugin({
                extractComments: true,
                minimizerOptions: {
                    minify: true,
                    mangle: true,
                    ecma: 5,
                    compress: {
                        passes: 2,
                    },
                    format: {
                        comments: false,
                    },
                },
            }),
            // Minify CSS
            new rspack.LightningCssMinimizerRspackPlugin({
                test: /\.css$/i,
            }),
            // Minify HTML
            new HtmlMinimizerPlugin({
                minify: HtmlMinimizerPlugin.swcMinify,
                minimizerOptions: {
                    scriptingEnabled: false,
                    collapseWhitespaces: "smart",
                    removeEmptyMetadataElements: true,
                    removeComments: true,
                    removeEmptyAttributes: true,
                    removeRedundantAttributes: true,
                    collapseBooleanAttributes: true,
                    normalizeAttributes: true,
                    sortSpaceSeparatedAttributeValues: true,
                    sortAttributes: true,
                },
            }),
        ],
    },
    module: {
        rules: [
            {
                test: /\.html$/i,
                type: "asset/resource",
            },
            {
                test: /\.css$/i,
                use: [rspack.CssExtractRspackPlugin.loader, "css-loader"],
                type: "javascript/auto",
            },
            {
                // Fixes: BREAKING CHANGE: The request '../../..' failed to resolve only because it was resolved as fully specified
                // See: https://stackoverflow.com/q/70964723
                test: /\.m?js/,
                resolve: {
                    fullySpecified: false,
                },
            },
            {
                test: /\.(j|t)s$/,
                exclude: [/[\\/]node_modules[\\/]/],
                loader: "builtin:swc-loader",
                options: {
                    jsc: {
                        parser: {
                            syntax: "typescript",
                        },
                        externalHelpers: false, // External helpers break the build
                        transform: {
                            react: {
                                runtime: "automatic",
                                development: !PROD,
                                refresh: !PROD,
                            },
                        },
                    },
                    env: {
                        targets: "Chrome >= 48", // browser compatibility
                    },
                },
            },
            {
                test: /\.(j|t)sx$/,
                exclude: [/[\\/]node_modules[\\/]/],
                loader: "builtin:swc-loader",
                options: {
                    jsc: {
                        parser: {
                            syntax: "typescript",
                            tsx: true,
                        },
                        externalHelpers: false, // External helpers break the build
                        transform: {
                            react: {
                                runtime: "automatic",
                                development: !PROD,
                                refresh: !PROD,
                            },
                        },
                    },
                    env: {
                        targets: "Chrome >= 48", // browser compatibility
                    },
                },
            },
        ],
    },
    plugins: [
        // Copy-pasta plugin
        // https://rspack.rs/plugins/rspack/copy-rspack-plugin
        new rspack.CopyRspackPlugin({
            patterns: [
                // { context: path.resolve(__dirname, GlobalPaths.ontology), from: "./*", to: `ontology` },
                {
                    context: path.resolve(__dirname, GlobalPaths.deploy),
                    from: path.resolve(
                        __dirname,
                        GlobalPaths.static + "/favicon.ico",
                    ),
                    to: ".",
                },
                {
                    context: path.resolve(__dirname, GlobalPaths.deploy),
                    from: "LICENSE",
                    to: ".",
                },
                {
                    context: path.resolve(__dirname, GlobalPaths.deploy),
                    from: path.resolve(
                        __dirname,
                        GlobalPaths.static + "/*.html",
                    ),
                    to: ".",
                },
            ],
        }),
        // Extracts CSS files
        // https://rspack.rs/plugins/rspack/css-extract-rspack-plugin
        new rspack.CssExtractRspackPlugin({ filename: "css/[name].css" }),
        // Compiles Rust to WebAssembly
        // https://github.com/WebVOWL/wasm-pack-plugin#usage
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, GlobalPaths.rustSrc),
            // Available arguments: https://github.com/WebVOWL/wasm-pack-plugin#usage
            args: "--verbose",
            extraArgs: "--target web --mode normal",
            // @ts-ignore
            forceMode: process.env.NODE_ENV,
            outDir: path.resolve(__dirname, GlobalPaths.pkgWasm),
            pluginLogLevel: "info",
            wasmInstaller: "rust",
        }),

        new rspack.HtmlRspackPlugin({
            title: NAME,
            template: `${GlobalPaths.static}/index.html`,
            scriptLoading: "module",
            minify: PROD,
            favicon: `${GlobalPaths.deploy}/favicon.ico`,
            meta: {
                description: `${NAME} - A performant and scalable WebVOWL`,
                robots: "noindex,nofollow",
                viewport:
                    "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no",
                "apple-mobile-web-app-capable": "yes",
            },
        }),
        // Constants available in HTML/JS/TS and replaced by Rspack during project build
        // https://rspack.rs/plugins/webpack/define-plugin
        new rspack.DefinePlugin({
            PROJECT_NAME: JSON.stringify(NAME),
            BUILD_TYPE: JSON.stringify(process.env.NODE_ENV),
            VERSION: JSON.stringify(pkg.version),
        }),
        // Cleans the build folder before building
        // https://github.com/johnagan/clean-webpack-plugin
        new CleanWebpackPlugin({
            dry: false,
            verbose: false,
            cleanOnceBeforeBuildPatterns: ["**/*"], // Relative to webpack's output.path directory.
        }),
        // Compresses files to improve page load times
        // https://github.com/webpack-contrib/compression-webpack-plugin
        new CompressionPlugin({
            filename: "[path][base].br",
            algorithm: "brotliCompress",
            compressionOptions: {
                //@ts-ignore
                params: {
                    [zlib.constants.BROTLI_PARAM_QUALITY]:
                        zlib.constants.BROTLI_MAX_QUALITY,
                },
            },
            minRatio: 0.8,
            deleteOriginalAssets: PROD,
        }),
    ],
});
