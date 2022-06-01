// const path = require("path");
const { SourceMapDevToolPlugin } = require("webpack");
// const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");
// const WebpackPwaManifest = require("webpack-pwa-manifest");

module.exports = {
  entry: "./index.js",
  mode: "none",
  module: {
    rules: [
      {
        test: /\.(scss)$/,
        use: [
          {
            // inject CSS to page
            loader: "style-loader"
          },
          {
            loader: MiniCssExtractPlugin.loader
          },
          {
            // translates CSS into CommonJS modules
            loader: "css-loader"
          },
          {
            // Run postcss actions
            loader: "postcss-loader"
          },
          {
            loader: "sass-loader"
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
    // new CleanWebpackPlugin(),
    new MiniCssExtractPlugin(),
    new HtmlWebpackPlugin({
      template: "index.html"
    })
  ]
};
