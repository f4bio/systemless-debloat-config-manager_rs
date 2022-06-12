const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { ServiceWorkerPlugin } = require("service-worker-webpack");

module.exports = {
  entry: [
    path.resolve(__dirname, "www", "index.js"),
    path.resolve(__dirname, "www", "style.css")
  ],
  plugins: [
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname)
    }),
    new MiniCssExtractPlugin(),
    new CopyPlugin({
      patterns: [
        { from: path.resolve(__dirname, "www", "assets", "app-icon.png") }
      ]
    }),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, "www", "index.html"),
      favicon: path.resolve(__dirname, "www", "favicon.ico"),
      title: "SystemlessDebloat Config Manager",
      meta: {
        author: "Fabio Tea",
        "application-name": "SystemlessDebloat Config Manager",
        "description": "SystemlessDebloat Config Manager",
        "apple-mobile-web-app-capable": "yes",
        "theme-color": "#1337af",
        "msapplication-TileColor": "#1337af",
        viewport: "width=device-width, initial-scale=1, shrink-to-fit=no"
      }
    }),
    new ServiceWorkerPlugin()
  ],
  experiments: {
    asyncWebAssembly: true
  },
  module: {
    rules: [
      {
        exclude: /node_modules/,
        test: /\.js$/,
        use: ["babel-loader"]
      },
      {
        test: /\.html$/,
        use: ["html-loader", "posthtml-loader"]
      },
      {
        test: /manifest\.json/,
        type: "asset/resource",
        generator: {
          filename: "manifest.json"
        }
      },
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"]
      },
      {
        test: /\.(png|svg|jpg|jpeg|gif)$/i,
        type: "asset/resource"
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: "asset/resource"
      }
    ]
  }
};
