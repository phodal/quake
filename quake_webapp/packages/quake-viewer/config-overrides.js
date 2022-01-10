const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const {override, disableEsLint} = require("customize-cra");

function overrideExtra(config, env) {
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
    filename: `static/quake-viewer.min.js`,
  };

  config.optimization.runtimeChunk = false;

  return config;
}

module.exports = overrideExtra
