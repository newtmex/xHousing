import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const chainID = 'T';

export const environment = EnvironmentsEnum.testnet;

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqdzsrrzf2mueyjs5ugq2mqr0gs2fgnxzj0fusrvzxkw';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgqutsyyrxp4vxjaya9jh79y8mh8enutrcq0fusf6fltz';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgqyuw45gja7rvrvfg8p798lne0xsau5g7a0fusl8phfz';
export const apiAddress = 'https://testnet-api.multiversx.com';
export const proxyAddress = 'https://testnet-gateway.multiversx.com';
