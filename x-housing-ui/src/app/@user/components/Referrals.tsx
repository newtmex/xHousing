import { xHousingSC } from '@/contracts/xHousing';
import { useGetAccount } from '@multiversx/sdk-dapp/hooks/account/useGetAccount';
import useSWR from 'swr';

import { Address } from '@multiversx/sdk-core/out';
import { truncateFromInside } from '@/utils';
import BlockiesImage from '@/components/BlockiesImage';

export default function Referrals() {
  const { address } = useGetAccount();
  const { data: referrals } = useSWR(
    address ? { key: 'Referrals-getUserReferrals', address } : null,
    ({ address }) =>
      xHousingSC
        .getAffiliateDetails({ address })
        .then((r) => xHousingSC.getUserReferrals({ id: r[0] }))
  );

  if (!referrals?.length) {
    return null;
  }

  return (
    <div className='element-wrapper compact pt-4'>
      <h6 className='element-header'>Your Referrals</h6>
      <div className='element-box-tp'>
        <div className='inline-profile-tiles'>
          <div className='row'>
            <div className='col-4 col-sm-3 col-xxl-2'>
              {referrals.map(([id, referralAddress]) => (
                <div
                  key={id + referralAddress}
                  className='profile-tile profile-tile-inlined'
                >
                  <a
                    className='profile-tile-box'
                    href='users_profile_small.html'
                  >
                    <div className='pt-avatar-w'>
                      <BlockiesImage
                        seed={new Address(referralAddress).pubkey().toString()}
                      />
                    </div>
                    <div className='pt-user-name'>
                      {truncateFromInside(referralAddress, 15)}
                    </div>
                  </a>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
