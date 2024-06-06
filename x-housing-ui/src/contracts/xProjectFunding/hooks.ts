import useSWRImmutable from 'swr';
import { xProjectFundingSC } from '.';

export const useLkXhtID = () =>
  useSWRImmutable('xProject-funding-LKXHT-id', () => xProjectFundingSC.getLkXhtID())
    .data;
