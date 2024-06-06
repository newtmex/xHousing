import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const chainID = 'T';

export const environment = EnvironmentsEnum.testnet;

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqx562aw8gmwuc6teas05kes8vv2zdhqwy0fusctp7xw';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgq8xpjv9fvcghtpmvmwx4v8j07rpu0eeep0fusz9w34v';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgqc8zzdp6ypp0ejg9ajpc595t8n4qn0etp0fusg6wy73';
export const apiAddress = 'https://testnet-api.multiversx.com';
export const proxyAddress = 'https://testnet-gateway.multiversx.com';
