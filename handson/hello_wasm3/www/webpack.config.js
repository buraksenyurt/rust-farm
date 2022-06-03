const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
  //entry: "./index.js",
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public"),
    //filename: "index.js"
    filename: "bootstrap.js"
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "./index.html", to: "./"}
      ]
    })
  ]
}
