import { defineConfig } from "@rspack/cli";
import path from "path";
import GlobalPaths from "./global-paths.ts";

/** Extra options for running a dev server */
export default defineConfig({
    devServer: {
        allowedHosts: "auto",
        host: "local-ip",
        port: "auto",
        server: "http",
        compress: false,
        hot: false,
        liveReload: true,
        open: true,
        setupExitSignals: true,
        headers: {
            "Cross-Origin-Resource-Policy": "cross-origin",
            "Cross-Origin-Opener-Policy": "same-origin",
            "Cross-Origin-Embedder-Policy": "require-corp",
        },
        static: {
            directory: path.resolve(GlobalPaths.deploy),
        },
        client: {
            overlay: {
                errors: true,
                warnings: false,
                runtimeErrors: true,
            },
            logging: "info",
            progress: true,
            reconnect: 3,
        },
        devMiddleware: {
            index: true,
            serverSideRender: true,
            writeToDisk: false,
            lastModified: true,
        },
    },
});
