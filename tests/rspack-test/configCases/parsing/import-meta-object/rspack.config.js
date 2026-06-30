/** @type {import("@rspack/core").Configuration} */
module.exports = {
  output: {
    filename: 'bundle.js',
  },
  module: {
    parser: {
      javascript: {
        importMetaContext: false,
        importMeta: {
          url: false,
          webpack: true,
          env: true,
          rspackHash: false,
          webpackContext: true,
        },
      },
    },
  },
};
