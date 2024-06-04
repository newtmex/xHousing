/** @type {import('next').NextConfig} */
const nextConfig = {
  distDir: 'build',
  transpilePackages: ['@multiversx/sdk-dapp'],
  async redirects() {
    return [
      {
        source: '/',
        destination: '/dashboard',
        permanent: true
      }
    ];
  },
  webpack: (config) => {
    config.resolve.fallback = { fs: false };
    config.externals.push('pino-pretty', 'lokijs', 'encoding', {
      bufferutil: 'bufferutil',
      'utf-8-validate': 'utf-8-validate'
    });

    return config;
  }
};
export default nextConfig;
