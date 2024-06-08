'use client';

import BlockiesImage from '@/components/BlockiesImage';
import { useGetAccountInfo } from '@/hooks';

export default function UserAvatar() {
  const { publicKey } = useGetAccountInfo();

  if (!publicKey) {
    return null;
  }

  return (
    <div className='avatar-w'>
      <BlockiesImage seed={publicKey} />
    </div>
  );
}
