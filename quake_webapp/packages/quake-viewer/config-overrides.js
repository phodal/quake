const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const {override, disableEsLint} = require("customize-cra");

function overrideExtra(config, env) {
  if (env === 'production') {
    // or adjustStyleLoaders

    // config.plugins.push(
    //   new MiniCssExtractPlugin({
    //     filename: "[name].[contenthash].css",
    //     chunkFilename: "[id].[contenthash].css",
    //   })
    // )
  }

  override(
    disableEsLint(),
  )(config, env);

  config.optimization.splitChunks = {
    cacheGroups: {
      default: false
    }
  };

  config.output = {
    ...config.output,
    filename: `static/quake-timeline.min.js`,
  };

  config.optimization.runtimeChunk = false;

  return config;
}

module.exports = overrideExtra
