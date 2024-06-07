import { EnvironmentsEnum } from '@/types';

export * from './sharedConfig';

export const chainID = 'T';

export const environment = EnvironmentsEnum.testnet;

export const coinbaseAddress =
  'erd1qqqqqqqqqqqqqpgqzpfdhdtq4p768ky8l47xnfrea2g4h9ck0fusadz42w';
export const xHousingAddress =
  'erd1qqqqqqqqqqqqqpgqv6fc7m9fymejw6ckykxhlcpw4w5txqqe0fus6sl8ye';
export const xProjectFundingAddress =
  'erd1qqqqqqqqqqqqqpgqtrp9ultkezqhc2fmy4wjnsy3pyvks2v90fusytuza0';
export const apiAddress = 'https://testnet-api.multiversx.com';
export const proxyAddress = 'https://testnet-gateway.multiversx.com';
