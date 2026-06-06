const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
    publicPath: "auto",
  },
  mode: "development",
  devServer: {
    proxy: [
      {
        context: ["/scoreboard", "/round"],
        target: "http://localhost:3000",
      },
    ],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: "index.html" }],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
};
