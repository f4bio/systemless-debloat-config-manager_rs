const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");
const path = require("path");

module.exports = merge(common, {
  mode: "development",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].bundle.js",
    publicPath: "/"
  },
  devtool: "inline-source-map",
  devServer: {
    liveReload: false,
    hot: false,
    static: "./dist"
  }
});
