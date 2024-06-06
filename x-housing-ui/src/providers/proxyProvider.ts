import { proxyAddress } from '@/config';
import { ProxyNetworkProvider } from '@multiversx/sdk-network-providers/out';

export const proxyProvider = new ProxyNetworkProvider(proxyAddress, {
  timeout: 20_000
});
