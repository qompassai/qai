const withBundleAnalyzer = require("@next/bundle-analyzer")({
  enabled: process.env.ANALYZE === "true",
});
const withPWA = require('next-pwa');

module.exports = withBundleAnalyzer(
  withPWA({
    pwa: {
      dest: 'public',
    },
    // Add any additional configuration here, for example:
    reactStrictMode: true,
    images: {
      domains: ['example.com'], // Example of adding images domain
    },
  })
);

