const path = require("path");
const { SourceMapDevToolPlugin } = require("webpack");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const FaviconsWebpackPlugin = require("favicons-webpack-plugin");

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
        test: /\.s[ac]ss$/i,
        use: ["style-loader", "css-loader", "sass-loader", "postcss-loader"]
      },
      {
        test: /\.(png|jpe?g|webp|git|svg|)$/i,
        use: [
          {
            loader: "img-optimize-loader"
          }
        ]
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/,
        loader: "file-loader",
        options: {
          outputPath: "fonts"
        }
      }
    ]
  },
  experiments: {
    asyncWebAssembly: true
  },
  plugins: [
    new SourceMapDevToolPlugin(),
    new CleanWebpackPlugin(),
    new FaviconsWebpackPlugin("./assets/esig.png"),
    // new CopyPlugin({
    //   patterns: [
    //     { from: "./assets/webassembly-icon.svg", to: "assets/" },
    //     { from: "./assets/rust-lang-icon.png", to: "assets/" },
    //   ],
    // }),
    new HtmlWebpackPlugin({
      template: "index.html"
    })
  ]
};
