import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const environment = EnvironmentsEnum.devnet;

export const chainID = 'D';

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqmepkngjhgw4kff4u976rmdpjpv3uc4mp0fuskv4y8j';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgqxaz0puawlrgazrq2d0qg58thd776fjaz0fusy7eqgv';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgq4qv8rgwxwmf9ujnlsxe2mgg9vgzdgjnu0fusn9thex';
export const apiAddress = 'https://devnet-api.multiversx.com';
export const proxyAddress = 'https://devnet-gateway.multiversx.com';
