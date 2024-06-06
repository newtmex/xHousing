import useSWRImmutable from 'swr/immutable';
import { coinbaseSC } from '.';

export const useXhtID = () =>
  useSWRImmutable('Coinbase-XHT-id', () => coinbaseSC.getXhtID()).data;
