import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const chainID = 'T';

export const environment = EnvironmentsEnum.testnet;

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqucyqvqgmyf0avp76evd7aqrxskvcj35e0fusnf5gfq';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgqs22spcvt0c4rl8zmfyufwxuqu2xneze40fus6t92fu';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgql5664ympcv9cfg7semytlsevw4jcu0gf0fusdshzex';
export const apiAddress = 'https://testnet-api.multiversx.com';
export const proxyAddress = 'https://testnet-gateway.multiversx.com';
