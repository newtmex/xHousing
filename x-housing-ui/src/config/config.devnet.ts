import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const environment = EnvironmentsEnum.devnet;

export const chainID = 'D';

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqmepkngjhgw4kff4u976rmdpjpv3uc4mp0fuskv4y8j';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgqs22spcvt0c4rl8zmfyufwxuqu2xneze40fus6t92fu';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgql5664ympcv9cfg7semytlsevw4jcu0gf0fusdshzex';
export const apiAddress = 'https://devnet-api.multiversx.com';
export const proxyAddress = 'https://devnet-gateway.multiversx.com';
