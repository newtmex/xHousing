import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const chainID = 'T';

export const environment = EnvironmentsEnum.testnet;

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqr4ux83pr2ptmkswd7ff3ghx3s9667av20fusu0hg0e';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgqag3drk2yhrk8z7gfaexu7hm23xe0wg750fus2fnfnl';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgqz7kdtyn5p60avldgqvmjschrayay5a4h0fusyff77x';
export const apiAddress = 'https://testnet-api.multiversx.com';
export const proxyAddress = 'https://testnet-gateway.multiversx.com';
