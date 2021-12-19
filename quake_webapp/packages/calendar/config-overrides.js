
module.exports = {
  webpack: function(config, env) {
    config.optimization.splitChunks = {
      cacheGroups: {
        default: false
      }
    };

    config.output = {
      ...config.output,
      filename: `static/quake-calendar.min.js`,
    };

    config.optimization.runtimeChunk = false;
    return config;
  }
}
