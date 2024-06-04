'use client';

import { useGetAccount, useGetIsLoggedIn } from '@/hooks';
import { truncateFromInside } from '@/utils';
import { CopyButton } from '@multiversx/sdk-dapp/UI/CopyButton/CopyButton';
import Link from 'next/link';

export default function UserInfo() {
  const isLoggedIn = useGetIsLoggedIn();
  const { username, address } = useGetAccount();

  return (
    <div className='logged-user-info-w'>
      {!isLoggedIn ? (
        <Link href='/unlock' className='mr-2 mb-2 btn btn-primary'>
          Connect&nbsp;Wallet
        </Link>
      ) : (
        <>
          <div className='logged-user-name'>
            {truncateFromInside(address, 15)}
            <CopyButton
              className='text-white ml-2'
              successIcon={'check'}
              text={address}
            />
          </div>
          {username && <div className='logged-user-role'>{username}</div>}
        </>
      )}
    </div>
  );
}
