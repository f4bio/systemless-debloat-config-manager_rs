const path = require("path");
const { SourceMapDevToolPlugin } = require("webpack");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// const WebpackPwaManifest = require("webpack-pwa-manifest");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js"
  },
  mode: "none",
  module: {
    rules: [
      {
        test: /\.(css)$/,
        use: [
          {
            loader: "style-loader"
          },
          {
            loader: "css-loader"
          },
          {
            loader: "postcss-loader"
          }
        ]
      },
      {
        test: /\.(jpe?g|png|gif|svg)$/i,
        type: "asset"
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/,
        loader: "file-loader"
      }
    ]
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new SourceMapDevToolPlugin(),
    new CleanWebpackPlugin(),
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin({
      template: "index.html"
    })
  ]
};
