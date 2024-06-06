import { apiAddress } from '@/config';
import { ApiNetworkProvider } from '@multiversx/sdk-network-providers/out';

export const apiProvider = new ApiNetworkProvider(apiAddress, {
  timeout: 20_000
});
