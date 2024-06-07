import { xHousingSC } from '@/contracts/xHousing';
import { RefIdData, getWindowLocation } from '@/utils';
import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import { useMemo } from 'react';
import useSWR from 'swr';
import { ReferralIDStructure } from './structure';
import { CopyButton } from '@multiversx/sdk-dapp/UI/CopyButton/CopyButton';

export default function ReferralCard() {
  const { address } = useGetAccount();
  const { data: refIdData } = useSWR(
    address ? { key: 'refdata-getAffiliateDetails', address } : null,
    ({ address }) =>
      xHousingSC
        .getAffiliateDetails({ address })
        .then(([userID]) => new RefIdData(address, userID))
  );
  const joinLink = useMemo(() => {
    return typeof window !== 'undefined' && refIdData?.refID
      ? `${getWindowLocation().origin}/${ReferralIDStructure.makeUrlSegment(
          refIdData.refID
        )}`
      : '';
  }, [refIdData]);

  if (!joinLink) {
    return null;
  }

  return (
    <div className='cta-w orange text-center'>
      <div className='cta-content extra-padded'>
        <div className='highlight-header'>Bonus</div>
        <h5 className='cta-header'>
          Invite your friends and make money with referrals
        </h5>
        <span className='badge bg-success'>
          <span id='ref_link'>{joinLink}</span>
        </span>
        <CopyButton className='text-white ml-2' text={joinLink} />
      </div>
    </div>
  );
}
