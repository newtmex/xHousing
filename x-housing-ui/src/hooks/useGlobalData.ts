import useSWR from 'swr';
import { useGetAccount } from './sdkDappHooks';
import { useEffect, useState } from 'react';
import { RefIdData } from '../utils';
import { getItem, setItem } from '@/storage/session';
import { xHousingSC } from '@/contracts/xHousing';

const useGlobalData = () => {
  // REF ID Stuff ---
  const [refID, setRefID] = useState<string | null | undefined>();
  const { data: refData, error } = useSWR(
    refID ? { key: 'getReferrerData', refID } : null,
    async ({ refID }) => {
      const id = RefIdData.getID(refID);
      const address = await xHousingSC.getUserAddress({ id });

      return new RefIdData(address, id);
    }
  );

  useEffect(() => {
    const url = window.location.toString().split('?')[1];
    const searchParams = new URLSearchParams(url);

    setRefID(searchParams.get('refID') || getItem('userRefBy'));
  }, []);
  useEffect(() => {
    if (refData?.refID) {
      typeof refData?.refID !== 'undefined' &&
        setItem({ key: 'userRefBy', data: refData.refID }, 60 * 60 * 24 * 360);
    }
  }, [refData, error]);
};

export default useGlobalData;
